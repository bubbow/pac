use std::process::Command;

use native_dialog::FileDialog;

use home::home_dir;

//Slint code
slint::slint! {
    import { Button, VerticalBox, ComboBox, LineEdit, HorizontalBox } from "std-widgets.slint";
    export global app_do {
        callback download_btn_pressed(string, string, int, int);
    }
    export component app {

        VerticalBox{
            Text {
                text: "Link";
            }
            file_link := LineEdit{
                placeholder-text: "e.g: https://youtu.be/dQw4w9WgXcQ?si=kM1TGCNp9D5S8RiJ";
            }
            Text{
                text: "Format";
            }
            HorizontalBox{
                format_index := ComboBox{
                    model: ["MP4", "AVI", "MOV", "MP3", "M4A", "WAV", "OGG"];
                    current-value: "MP4";
                }
                quality_index := ComboBox{
                    model: ["4k", "1440p", "1080p", "720p", "480p", "360p", "240p", "144p"];
                    current-value: "1080p";
                }
            }
            Text{
                text: "Output file name";
            }
            file_name := LineEdit {
                placeholder-text: "e.g: rickroll (or leave blank)";

            }
            download_btn := Button{
                text: "Download";
                clicked => {app-do.download_btn_pressed(file-link.text, file-name.text, format-index.current-index, quality_index.current-index)}
            }

        }
    }
}

//Main function
fn main() {
    
    let ui = app::new().unwrap();

    let path = home_dir().map_or("~".to_string(), |p| p.display().to_string());

    let shell = if cfg!(target_os = "windows") {
        "cmd"
    } else {
        "sh"
    };
    let flag = if cfg!(target_os = "windows") {
        "/C"
    } else {
        "-c"
    };
    let os_slash = if cfg!(target_os = "windows") {
        "\\"
    } else {
        "/"
    };
    

    ui.global::<app_do>()
        .on_download_btn_pressed(move |file_link, file_name, format_index, quality_index| {

            let quality = parse_format_index_quality(quality_index, format_index);
            let format_codec = parse_format_index_codec(format_index);

            let path = FileDialog::new()
                .set_location(&path)
                .show_open_single_dir()
                .unwrap();
            //Executing the yt-dlp command, codes a bit messy
            if file_name == "" {
                Command::new(shell)
                    .arg(flag)
                    .arg(format!(
                        "yt-dlp {} -I 1 -f '{}' --recode {} -P '{}'",
                        quality,
                        file_link,
                        format_codec,
                        path.expect("Failed to get path").display()
                    ))
                    .output()
                    .expect("Failed to execute yt-dlp, Do you have it installed?");
            } else {
                Command::new(shell)
                    .arg(flag)
                    .arg(format!(
                        "yt-dlp -I 1 -f '{}' '{}' --recode {} -o '{}{}{}'",
                        quality,
                        file_link,
                        format_codec,
                        path.expect("Failed to get path").display(),
                        os_slash,
                        file_name
                    ))
                    .output()
                    .expect("Failed to execute yt-dlp, Do you have it installed?");
            }
            
            println!("The command below isnt really the one being executed, it's just here to tell me what is happening");
            println!("{}", format!(
                "yt-dlp -I 1 -f '{}' '{}' --recode {} -o '{}{}'",
                quality,
                file_link,
                format_codec,
                os_slash,
                file_name
            ))
        });

    ui.run().unwrap();
}

fn parse_format_index_quality(quality_index: i32, format_index: i32) -> String {
    //quality_index, 0=4k 1=1440p 2=1080p 3=720p 4=480p 5=360p 6=240p 7=144p
    let quality_command = match quality_index {
        0 => "[height<=2160]",
        1 => "[height<=1440]",
        2 => "[height<=1080]",
        3 => "[height<=720]",
        4 => "[height<=480]",
        5 => "[height<=360]",
        6 => "[height<=240]",
        7 => "[height<=144]",
        _ => "[height<=1080]"
    };
    //format_index, 0=mp4 1=avi 2=mov 3=mp3 4=m4a 5=wav 6=ogg
    let quality = match format_index{
        
        0 | 1 | 2 => format!("bestvideo{}+bestaudio", quality_command),
        _ => "bestaudio".to_string()
    };

    println!("{}", format!("Quality: Quality indexed as {}, Format indexed as {}, parsing to {}.", 
    quality_index,
    format_index,
    quality
    ));
    return quality.to_string();
}

fn parse_format_index_codec(format_index: i32) -> String {
    //format_index, 0=mp4 1=avi 2=mov 3=mp3 4=m4a 5=wav 6=ogg
    let format_codec = match format_index {
        0 => "mp4",
        1 => "avi",
        2 => "mov",
        3 => "mp3",
        4 => "m4a",
        5 => "wav",
        6 => "ogg",
        _ => "mp4",
    };
    println!("{}", format!("Codec: Format indexed as {}, parsing to {}.",
    format_index,
    format_codec
    ));
    return format_codec.to_string();
}



