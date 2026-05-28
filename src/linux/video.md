# 视频压缩

kdenlive 视频导出体积较大，需要压缩后再上传。

## 通常使用-平衡画质与大小

```bash
ffmpeg -hwaccel cuda -i input.mp4 \
       -c:v hevc_nvenc -preset p7 -rc vbr -cq 32 -b:v 0 \
       -c:a aac -b:a 128k \
       output.mp4
```

-hwaccel cuda：调用 CUDA 加速解码。

-c:v hevc_nvenc：指定使用 NVIDIA 的 HEVC 硬件编码器。

-preset p7：NVENC 的最高质量预设，P1到P7，P7质量最高但速度最慢。

-rc vbr：使用可变码率(VBR)模式控制编码。

-cq 32：硬件编码下的“恒定质量”参数，类似于 -crf，推荐范围 28-35。

-b:v 0：与 -cq 配合使用，让编码器完全关注于恒定质量而非码率。

## 追求最小体积-压缩时间很长

```bash
ffmpeg -i input.mp4 \
       -c:v libx265 -crf 28 -preset slower \
       -x265-params "aq-mode=3:no-sao=1:selective-sao=2" \
       -vf "unsharp=5:5:1.0:5:5:0.0" \
       -c:a libopus -b:a 96k \
       output.mkv
```

-preset slower：用更长的处理时间，换来更好的压缩效果。

-x265-params：为 libx265 编码器提供高级微调参数，以优化画面细节或编码效率。

-vf "unsharp=..."：应用锐化滤镜，在低码率下提升画质观感。

-c:a libopus -b:a 96k：使用高效的 Opus 音频编码，96kbps 的立体声音质已相当出色。
