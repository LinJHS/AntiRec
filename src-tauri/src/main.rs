// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample, SampleFormat, StreamConfig};
use dasp_sample::conv::ToSample;
use directories::BaseDirs;
use hound::{SampleFormat as HoundSampleFormat, WavSpec, WavWriter};
use lazy_static::lazy_static;
use serde::Serialize;
use std::fs::File;
use std::io::BufWriter;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::Window;

#[derive(Clone, Serialize)]
struct Payload<T> {
    ori: Vec<T>,
    new: Vec<T>,
}

lazy_static! {
    static ref HAS_RUN_AUDIO: AtomicBool = AtomicBool::new(false);
}

#[tauri::command]
fn audio_process(add_values: Vec<f32>, window: Window) -> () {
    HAS_RUN_AUDIO.store(true, Ordering::SeqCst);
    thread::spawn(move || {
        // 获取当前时间
        let now = SystemTime::now();
        let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let in_ms = since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_millis() as u64;

        let base_dirs = BaseDirs::new().unwrap();
        let path = base_dirs.data_dir().to_str().unwrap();
        // 创建文件
        let input_filepath = format!("{}/top.linjhs.anti-rec/waves/{}_ori.wav", path, in_ms);
        let output_filepath = format!("{}/top.linjhs.anti-rec/waves/{}_new.wav", path, in_ms);

        // 获取默认的 host
        let host = cpal::default_host();

        // 获得默认输入输出设备
        let input_device = host
            .default_input_device()
            .expect("Failed to get default input device");

        let mut output_device = host
            .default_output_device()
            .expect("Failed to get default output device");

        let output_device_list = host.output_devices().unwrap();

        for device in output_device_list {
            if device.name().unwrap() == "Line 1 (Virtual Audio Cable)" {
                output_device = device;
            }
        }

        println!("Start Audio Process");

        // 获取输入输出配置
        let mut input_supported_configs_range = input_device
            .supported_input_configs()
            .expect("error while querying configs");

        let input_supported_configs = input_supported_configs_range
            .next()
            .expect("no supported config?!")
            .with_max_sample_rate();

        let mut output_supported_configs_range = output_device
            .supported_output_configs()
            .expect("error while querying configs");

        let output_supported_configs = output_supported_configs_range
            .next()
            .expect("no supported config?!")
            .with_max_sample_rate();

        // 音频数据缓冲区
        let buffer = Arc::new(Mutex::new(Vec::new()));
        let add_index = Arc::new(Mutex::new(0));

        // 输入部分
        let input_err_fn = |err| eprintln!("an error occurred on the input audio stream: {}", err);
        let input_sample_format = input_supported_configs.sample_format();
        let input_config: StreamConfig = input_supported_configs.into();
        let input_buffer = Arc::clone(&buffer);
        let input_add_index = Arc::clone(&add_index);

        println!("声道数：{}", input_config.channels);
        println!("采样率：{}", input_config.sample_rate.0);

        // 创建 WAV 文件写入器
        let input_spec = WavSpec {
            channels: input_config.channels,
            sample_rate: input_config.sample_rate.0 / 3,
            bits_per_sample: 16,
            sample_format: HoundSampleFormat::Int,
        };
        let mut input_writer = Arc::new(Mutex::new(
            WavWriter::create(input_filepath, input_spec).expect("Failed to create WAV file"),
        ));
        // let input_writer_clone = Arc::clone(&input_writer);

        // 创建输入流
        let input_stream = match input_sample_format {
            SampleFormat::F32 => input_device.build_input_stream(
                &input_config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    input_callback::<f32>(
                        data,
                        &input_buffer,
                        &input_add_index,
                        &add_values,
                        &mut input_writer,
                        &window,
                    );
                },
                input_err_fn,
                None,
            ),
            SampleFormat::I16 => input_device.build_input_stream(
                &input_config,
                move |data: &[i16], _: &cpal::InputCallbackInfo| {
                    input_callback::<i16>(
                        data,
                        &input_buffer,
                        &input_add_index,
                        &add_values,
                        &mut input_writer,
                        &window,
                    );
                },
                input_err_fn,
                None,
            ),
            SampleFormat::U16 => input_device.build_input_stream(
                &input_config,
                move |data: &[u16], _: &cpal::InputCallbackInfo| {
                    input_callback::<u16>(
                        data,
                        &input_buffer,
                        &input_add_index,
                        &add_values,
                        &mut input_writer,
                        &window,
                    );
                },
                input_err_fn,
                None,
            ),
            SampleFormat::U8 => input_device.build_input_stream(
                &input_config,
                move |data: &[u8], _: &cpal::InputCallbackInfo| {
                    input_callback::<u8>(
                        data,
                        &input_buffer,
                        &input_add_index,
                        &add_values,
                        &mut input_writer,
                        &window,
                    );
                },
                input_err_fn,
                None,
            ),
            sample_format => panic!("Unsupported sample format '{sample_format}'"),
        }
        .unwrap();

        // 输出部分
        let output_err_fn =
            |err| eprintln!("an error occurred on the output audio stream: {}", err);
        let output_sample_format = output_supported_configs.sample_format();
        let output_config: StreamConfig = output_supported_configs.into();
        let output_buffer = Arc::clone(&buffer);

        // 创建 WAV 文件写入器
        let output_spec = WavSpec {
            channels: input_config.channels,
            sample_rate: output_config.sample_rate.0 / 3,
            bits_per_sample: 16,
            sample_format: HoundSampleFormat::Int,
        };
        let mut output_writer = Arc::new(Mutex::new(
            WavWriter::create(output_filepath, output_spec).expect("Failed to create WAV file"),
        ));
        // let output_writer_clone = Arc::clone(&output_writer);

        // 创建输出流
        let output_stream = match output_sample_format {
            SampleFormat::F32 => output_device.build_output_stream(
                &output_config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    output_callback::<f32>(data, &output_buffer, &mut output_writer);
                },
                output_err_fn,
                None,
            ),
            SampleFormat::I16 => output_device.build_output_stream(
                &output_config,
                move |data: &mut [i16], _: &cpal::OutputCallbackInfo| {
                    output_callback::<i16>(data, &buffer, &mut output_writer);
                },
                output_err_fn,
                None,
            ),
            SampleFormat::U16 => output_device.build_output_stream(
                &output_config,
                move |data: &mut [u16], _: &cpal::OutputCallbackInfo| {
                    output_callback::<u16>(data, &buffer, &mut output_writer);
                },
                output_err_fn,
                None,
            ),
            SampleFormat::U8 => output_device.build_output_stream(
                &output_config,
                move |data: &mut [u8], _: &cpal::OutputCallbackInfo| {
                    output_callback::<u8>(data, &buffer, &mut output_writer);
                },
                output_err_fn,
                None,
            ),
            sample_format => panic!("Unsupported sample format '{sample_format}'"),
        }
        .unwrap();

        input_stream.play().unwrap();
        output_stream.play().unwrap();

        loop {
            if HAS_RUN_AUDIO.load(Ordering::SeqCst) {
                thread::sleep(Duration::from_secs(1));
            } else {
                println!("Quit thread");
                break;
            }
        }

        input_stream.pause().unwrap();
        output_stream.pause().unwrap();

        // 完成后调用 finalize
        // {
        //     let input_writer_lock = input_writer_clone.lock().unwrap();
        //     input_writer_lock.finalize().unwrap();
        // }
        // {
        //     let output_writer_lock = output_writer_clone.lock().unwrap();
        //     output_writer_lock.finalize().unwrap();
        // }
    });
}

fn input_callback<T>(
    data: &[T],
    buffer: &Arc<Mutex<Vec<f32>>>,
    add_index: &Arc<Mutex<usize>>,
    add_values: &[f32],
    writer: &mut Arc<Mutex<WavWriter<BufWriter<File>>>>,
    window: &Window,
) where
    T: Sample + ToSample<f32>,
{
    // 先上互斥锁，防止数据竞争
    let mut buffer = buffer.lock().unwrap();
    let mut add_index = add_index.lock().unwrap();
    let mut add_index_channel = 0;

    let mut data_ori: Vec<f32> = Vec::new();
    let mut data_new: Vec<f32> = Vec::new();
    let mut save_index: i8 = 0;

    // 对获取到的每一位音频数据进行逐位处理
    for &sample in data.iter() {
        save_index += 1;
        let normalized_sample = sample.to_sample::<f32>();
        let modified_sample = normalized_sample + add_values[*add_index];
        // 保存原始音频数据和处理过后的音频数据
        data_ori.push(normalized_sample);
        data_new.push(modified_sample);
        buffer.push(modified_sample);

        add_index_channel += 1;
        if add_index_channel == 2 {
            add_index_channel = 0;
            *add_index = (*add_index + 1) % add_values.len();
        }

        if save_index == 3 {
            // 将样本转换为16位整数并写入WAV文件
            let sample_i16 = (normalized_sample * std::i16::MAX as f32)
                .clamp(std::i16::MIN as f32, std::i16::MAX as f32)
                as i16;
            writer
                .lock()
                .unwrap()
                .write_sample(sample_i16)
                .expect("Failed to write sample");
            save_index = 0;
        }
    }

    let _ = window
        .emit(
            "audio_update",
            Payload {
                ori: data_ori,
                new: data_new,
            },
        )
        .unwrap();
}

fn output_callback<T>(
    data: &mut [T],
    buffer: &Arc<Mutex<Vec<f32>>>,
    writer: &mut Arc<Mutex<WavWriter<BufWriter<File>>>>,
) where
    T: Sample + FromSample<f32>,
{
    // 先上互斥锁，防止数据竞争
    let mut buffer = buffer.lock().unwrap();
    let mut save_index: i8 = 0;

    for sample in data.iter_mut() {
        // 从缓冲区中取出数据输出
        if let Some(output_sample) = buffer.pop() {
            *sample = T::from_sample(output_sample);
            save_index += 1;

            if save_index == 3 {
                // 将样本转换为16位整数并写入WAV文件
                let sample_i16 = (output_sample * std::i16::MAX as f32)
                    .clamp(std::i16::MIN as f32, std::i16::MAX as f32)
                    as i16;
                writer
                    .lock()
                    .unwrap()
                    .write_sample(sample_i16)
                    .expect("Failed to write sample");
                save_index = 0;
            }
        } else {
            *sample = Sample::EQUILIBRIUM;
        }
    }
}

#[tauri::command]
fn audio_stop() -> () {
    // 关闭线程
    HAS_RUN_AUDIO.store(false, Ordering::SeqCst);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![audio_process, audio_stop])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
