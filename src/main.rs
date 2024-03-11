use std::process::Command;

use native_dialog::FileDialog;

use home::home_dir;

//Slint code
slint::slint!{
    import { Button, VerticalBox, ComboBox, LineEdit } from "std-widgets.slint";
    export global app_do {
        callback download_btn_pressed(string, string, int);     
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
            format_index := ComboBox{
                model: ["MP4", "AVI", "MOV", "MP3", "M4A", "WAV", "OGG"];
                current-value: "MP4"; 
                
            }
            Text{
                text: "Output file name";
            }
            file_name := LineEdit {
                placeholder-text: "e.g: rickroll (or leave blank)";

            }
            download_btn := Button{
                text: "Download";
                clicked => {app-do.download_btn_pressed(file-link.text, file-name.text, format-index.current-index)}
            }
            
        }
    }
}

//Main function
fn main() {
    
    let path = home_dir().map_or("~".to_string(), |p| p.display().to_string()); 
    let ui = app::new().unwrap();

    let shell = if cfg!(target_os = "windows"){"cmd"}else{"sh"};
    let flag = if cfg!(target_os = "windows"){"/C"}else{"-c"};
    let osslash = if cfg!(target_os = "windows"){"\\"}else{"/"};

    ui.global::<app_do>().on_download_btn_pressed(move |string, String, int| 
        {             
            let file_link = string;
            let file_name = String;
            let format_index = int;
            //Lets get the combo, assume the user wants audio in their video
            let download_type_combo = match format_index{
                0|1|2 => "bestvideo+bestaudio",
                3|4|5|6 => "bestaudio",
                _ => "bestvideo+bestaudio"
            };
            let path = FileDialog::new()
                .set_location(&path)
                .show_open_single_dir()
                .unwrap();
 
             /* 
            file_format, 0=mp4 1=avi 2=mov 3=mp3 4=m4a 5=wav 6=ogg
             */

            //Lets get the file format
            let download_type_format = match format_index{
                0 => "mp4",
                1 => "avi",
                2 => "mov",
                3 => "mp3",
                4 => "m4a",
                5 => "wav",
                6 => "ogg",
                _ => "mp4"
            };
            //Executing the yt-dlp command, codes a bit messy
            if file_name == ""{
             Command::new(shell).arg(flag)
                .arg(format!("yt-dlp -I 1 -f  '{}' '{}' --recode {} -P '{}'",
                    download_type_combo, 
                    file_link, 
                    download_type_format, 
                    path.expect("Failed to get path").display()
                    )).output().expect("Failed to execute yt-dlp, Do you have it installed?");
            }else{
                Command::new(shell).arg(flag)
                    .arg(format!("yt-dlp -I 1 -f  '{}' '{}' --recode {} -o '{}{}{}'",
                        download_type_combo, 
                        file_link, 
                        download_type_format, 
                        path.expect("Failed to get path").display(), 
                        osslash, 
                        file_name)).output().expect("Failed to execute yt-dlp, Do you have it installed?");
            }
        });
    
    ui.run().unwrap();
}



