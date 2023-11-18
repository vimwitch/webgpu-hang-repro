use std::{borrow::Cow};
use std::time::{Instant, Duration};

#[cfg(not(test))]
fn main() {
    pollster::block_on(run());
}

#[cfg_attr(test, allow(dead_code))]
async fn run() {
    let instance = wgpu::Instance::default();

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await.unwrap();

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::downlevel_defaults(),
            },
            None,
        )
        .await
        .unwrap();

    let cs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
    });

    ////////
    // Program hangs here
    ////////
    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: None,
        module: &cs_module,
        entry_point: "test_mul",
    });
    let time = execute_gpu(&pipeline, &device, &queue).await;
    println!("gpu: {:.2?}", time);
}

async fn execute_gpu(
    compute_pipeline: &wgpu::ComputePipeline,
    device: &wgpu::Device,
    queue: &wgpu::Queue
) -> Duration {
    let gpu_start = Instant::now();
    // Gets the size in bytes of the buffer.

    // let input1_storage_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    //     label: Some("Storage Buffer"),
    //     contents: bytemuck::cast_slice(&[0; 262144]),
    //     usage: wgpu::BufferUsages::STORAGE
    //         | wgpu::BufferUsages::COPY_DST,
    // });

    // Instantiates the bind group, once again specifying the binding of buffers.
    // let bind_group_layout = compute_pipeline.get_bind_group_layout(0);
    // let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
    //     label: None,
    //     layout: &bind_group_layout,
    //     entries: &[
    //     //     wgpu::BindGroupEntry {
    //     //     binding: 0,
    //     //     resource: input1_storage_buffer.as_entire_binding(),
    //     // }
    //     ],
    // });

    // A command encoder executes one or many pipelines.
    // It is to WebGPU what a command buffer is to Vulkan.
    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: None,
            timestamp_writes: None,
        });
        cpass.set_pipeline(compute_pipeline);
        // cpass.set_bind_group(0, &bind_group, &[]);
        cpass.insert_debug_marker("modular multiplication");
        cpass.dispatch_workgroups(16, 1, 1); // Number of cells to run, the (x,y,z) size of item being processed
    }

    // Submits command encoder for processing
    queue.submit(Some(encoder.finish()));

    // Poll the device in a blocking manner so that our future resolves.
    // In an actual application, `device.poll(...)` should
    // be called in an event loop or on another thread.
    device.poll(wgpu::Maintain::Wait);
    gpu_start.elapsed()
}
