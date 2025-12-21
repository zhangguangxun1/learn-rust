use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Write};
use std::time::{Duration, Instant};
use std::{fs, thread};
use std::path::Path;
use lofty::picture::MimeType;
use lofty::prelude::TaggedFileExt;
use lofty::probe::Probe;
use lofty::tag::Accessor;

///
/// rodio 需要依赖下面两个库, Debian 13
/// libasound2-dev: 提供了 Rust 编译时需要的 C 语言头文件和底层接口，用于控制音频输出。
/// pkg-config: Rust 的构建脚本（build.rs）需要这个工具来定位系统库的安装路径。
/// 播放完毕会自己结束运行
///

// 歌词结构体
struct LyricLine {
    time: Duration,
    text: String,
}

// 单元测试是默认不实时输出 println! 宏, 所以要用住方法运行
// 这些符号是 LRC 歌词文件（LyRic 文件）中的标签（ID Tags）。
// 它们主要用于记录这首歌的元数据（Metadata），即歌曲的基本信息，类似于音乐文件的“身份证”。
// 以下是各个标签的具体含义：
// [ti:] (Title)：代表歌名。
// 例如：[ti:七里香]
// [ar:] (Artist)：代表歌手/艺术家。
// 例如：[ar:周杰伦]
// [al:] (Album)：代表专辑名称。
// 例如：[al:范特西]
// [by:] (By)：代表歌词制作人或编辑者。
// 在你提供的例子中，[by:天琴实验室AI生成v1.0] 表示这段歌词是由腾讯天琴实验室的 AI 算法自动生成的，而不是人工编写。
// [offset:] (时间偏移量)：代表整体时间补偿。
// 单位是毫秒（ms）。
// 如果设置为 [offset:500]，表示歌词整体延后 0.5 秒显示；如果是负数，则提前显示。用于微调歌词与声音的同步。
pub fn test_local_music() {
    let stream_handle =
        rodio::OutputStreamBuilder::open_default_stream().expect("Failed to open default stream");
    let sink = rodio::Sink::connect_new(&stream_handle.mixer());

    // 多首歌要用循环的, 这样就能控制先播放哪首歌, 然后再处理跟这首歌相关的 暂停 跳过 调整进度等等等操作
    let mut music_file_map: HashMap<&str, &str> = HashMap::new();
    music_file_map.insert("蔷薇团长-天上明月心上人.mp3", "蔷薇团长-天上明月心上人.lrc");
    music_file_map.insert("枝上桃-别过之后.flac", "枝上桃-别过之后.lrc");
    for file in music_file_map.iter() {
        sink.append(
            rodio::Decoder::new(BufReader::new(
                File::open(format!("/home/zhangguangxun/Music/{}", file.0))
                    .expect("Failed to open music file"),
            ))
            .unwrap(),
        );
        // 解析歌词
        let lrc_content = fs::read_to_string(format!("/home/zhangguangxun/Music/{}", file.1))
            .expect("Failed to read file");
        let mut lyrics = get_lry_lines(&lrc_content);

        // 开始记时
        let start_time = Instant::now();

        println!("开始播放");

        while !sink.empty() {
            let elapsed = start_time.elapsed();

            if let Some(pos) = lyrics.iter().position(|x| x.time <= elapsed) {
                let line = lyrics.remove(pos);
                println!("{}", line.text);
            }

            // 稍微休眠，减少 CPU 占用
            thread::sleep(Duration::from_millis(50));
        }
    }
}

fn get_lry_lines(content: &str) -> Vec<LyricLine> {
    let mut lines = Vec::new();
    for line in content.lines() {
        if line.starts_with("[0") && line.len() > 10 {
            let minutes: u64 = line[1..3].parse().unwrap_or(0);
            let seconds: f64 = line[4..9].parse().unwrap_or(0.0);
            let total_ms = (minutes * 60 * 1000) + (seconds * 1000.0) as u64;

            let text = line[10..].trim().to_string();
            lines.push(LyricLine {
                time: Duration::from_millis(total_ms),
                text,
            })
        }
    }

    // 排序确保顺序正确
    lines.sort_by(|a, b| a.time.cmp(&b.time));

    lines
}

#[cfg(test)]
#[test]
fn test_get_lry_lines() {
    // 解析歌词
    let lrc_content = fs::read_to_string("/home/zhangguangxun/Music/蔷薇团长-天上明月心上人.lrc")
        .expect("Failed to read file");
    let lyrics = get_lry_lines(&lrc_content);
    lyrics
        .iter()
        .for_each(|lyric| println!("{:?}: {}", lyric.time, lyric.text));
}

// 标题: Some("天上明月心上人")
// 艺术家: Some("蔷薇团长")
// 专辑: Some("天上明月心上人")
// TrackTitle, Text("天上明月心上人")
// AlbumTitle, Text("天上明月心上人")
// TrackArtist, Text("蔷薇团长")
//
// 标题: Some("茶汤")
// 艺术家: Some("郁可唯")
// 专辑: Some("微加幸福")
// AlbumTitle, Text("微加幸福")
// TrackArtist, Text("郁可唯")
// TrackTitle, Text("茶汤")
// EncoderSoftware, Text("reference libFLAC 1.3.1 20141125")
#[cfg(test)]
#[test]
fn test_read_tag() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 探测文件格式并读取标签
    // let path = Path::new("/home/zhangguangxun/Music/蔷薇团长-天上明月心上人.mp3");
    let path = Path::new("/home/zhangguangxun/Music/郁可唯-茶汤.flac");
    let tagged_file = Probe::open(path)?.guess_file_type()?.read()?;

    // 2. 获取主标签（如 ID3v2 或 Vorbis Comment）
    if let Some(tag) = tagged_file.primary_tag() {
        // 获取基本信息
        println!("标题: {:?}", tag.title());
        println!("艺术家: {:?}", tag.artist());
        println!("专辑: {:?}", tag.album());
        println!("类型: {:?}", tag.genre());
        println!("comment: {:?}", tag.comment());

        // 3. 获取内嵌歌词 (通常存储在 USLT 帧或 LYRICS 字段)
        // Lofty 提供了解析常用标签的统一方法
        for item in tag.items() {
            // 常见的歌词键名包括 "lyrics", "unsynchronizedLyrics" 等
            println!("{:?}, {:?}", item.key(), item.value())
        }

        // 3. 获取封面图片（通常第一张就是封面）
        // tag.pictures(): 这个方法会返回一个图片列表。因为一个音乐文件可能包含多张图片（如封面、封底、歌手写真、插图等），通常索引为 0 的（first()）就是专辑主封面。
        // picture.data(): 返回的是原始的 &[u8] 字节流。你可以直接将其写入文件，或者传给图形库（如 image crate）进行处理。
        // picture.mime_type(): 帮助你判断图片的真实格式。有些文件后缀是 .mp3，但内部嵌入的可能是 PNG 格式的封面。
        // PictureType: 你还可以通过 picture.pic_type() 来判断这张图的具体用途（例如 PictureType::CoverFront 代表正封面）。
        println!("pictures: {:?}", tag.pictures());
        if let Some(picture) = tag.pictures().first() {
            // 获取图片的二进制数据
            let image_data = picture.data();

            // 获取图片的 MIME 类型（例如 "image/jpeg" 或 "image/png"）
            let mime_type = picture.mime_type();
            let extension = match mime_type {
                Some(MimeType::Jpeg) => "jpg",
                Some(MimeType::Png) => "png",
                _ => "bin", // 无法识别时默认后缀
            };

            // 4. 将图片数据写入文件
            let output_name = format!("cover.{}", extension);
            let mut file = File::create(&output_name)?;
            file.write_all(image_data)?;

            println!("封面已成功提取并保存为: {}", output_name);
            println!("图片类型: {:?}", mime_type);
        } else {
            println!("该音频文件中没有找到嵌入的封面图片。");
        }
    } else {
        println!("该文件未包含任何元数据标签。");
    }

    Ok(())
}
