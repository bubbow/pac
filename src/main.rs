use std::{process::Command, string};

use native_dialog::FileDialog;



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
            file_format := ComboBox{
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
                clicked => {app-do.download_btn_pressed(file-link.text, file-name.text, file-format.current-index)}
            }
            
        }
    }
}

//Main function
fn main() {
    
    let path = if cfg!(target_os = "windows"){"C:\\"}else{"~"};

    let ui = app::new().unwrap();
    
    ui.global::<app_do>().on_download_btn_pressed(move |string, String, int| 
        {             
            //this thing... *excuse my langauge* is f**king horrible but it works
            let file_link = string;
            let file_name = String;
            let file_format = int;
            let download_type_combo = if file_format == 0 {"bestvideo+bestaudio"}else{"bestaudio"};
            let download_type_combo = if file_format == 1 {"bestvideo+bestaudio"}else{download_type_combo};
            let download_type_combo = if file_format == 2 {"bestvideo+bestaudio"}else{download_type_combo};
            
            let path = FileDialog::new()
                .set_location(path)
                .show_open_single_dir()
                .unwrap();

            let download_type_format = if file_format == 0 {"mp4"}else{"mp4"};
            let download_type_format = if file_format == 1 {"avi"}else{download_type_format};
            let download_type_format = if file_format == 2 {"mov"}else{download_type_format};
            let download_type_format = if file_format == 3 {"mp3"}else{download_type_format};
            let download_type_format = if file_format == 4 {"m4a"}else{download_type_format};
            let download_type_format = if file_format == 5 {"wav"}else{download_type_format};
            let download_type_format = if file_format == 6 {"ogg"}else{download_type_format};
            /* 
            file_format, 0=mp4 1=avi 2=mov 3=mp3 4=m4a 5=wav 6=ogg
             */
            //theres a better way to do this, i'll fix later
            if cfg!(target_os = "windows") && file_name == ""{
                Command::new("cmd").arg("/C")
                .arg(format!("yt-dlp -I 1 -f '{}' '{}' --recode {} -P '{}'", 
                download_type_combo, file_link, download_type_format, path.expect("fail 1").display())).output().expect("fail 2");
            }
            else if cfg!(target_os = "windows"){ 
                Command::new("cmd").arg("/C")
                .arg(format!("yt-dlp -I 1 -f '{}' '{}' --recode {} -o '{}\\{}'",
                download_type_combo, file_link, download_type_format, path.expect("fail 1").display(), file_name)).output().expect("fail 2");
            }
            else if file_name == ""
            {
                Command::new("sh").arg("-c")
                .arg(format!("yt-dlp -I 1 -f  '{}' '{}' --recode {} -P '{}'",
                 download_type_combo, file_link, download_type_format, path.expect("fail 1").display())).output().expect("fail 2");
            }else
            {
                Command::new("sh").arg("-c")
                .arg(format!("yt-dlp -I 1 -f  '{}' '{}' --recode {} -o '{}/{}'",
                 download_type_combo, file_link, download_type_format, path.expect("fail 1").display(), file_name)).output().expect("fail 2");
            }
            
        });
    
    ui.run().unwrap();
}

