use std::io::{self, Write};
use std::process::Command;

fn main() {
    println!("=== Image Series Converter (ffmpeg wrapper) ===");
    println!("NOTE: This program requires ffmpeg to be installed.");
    println!("  - Either install ffmpeg and make sure it is in your PATH");
    println!("  - Or place the ffmpeg executable in the same folder as this program.");
    println!("Download: https://ffmpeg.org/download.html\n");

    loop {
        // 1) Ask for prefix & suffix
        println!("\nStep 1: Frame name pattern");

        let prefix = prompt("Enter file prefix (before frame number, e.g. 'S_Film.'): ");
        let suffix = prompt("Enter file suffix (after frame number, e.g. '_5p.png'): ");

        let input_pattern = format!("{}%04d{}", prefix, suffix);
        println!("→ Using input pattern: {}", input_pattern);
        println!("  (This expects files like: {}0000{} , {}0001{} , ...)",
                 prefix, suffix, prefix, suffix);

        // 2) FPS
        println!("\nStep 2: Frame rate");
        let fps_str = prompt("Enter FPS (default 24): ");
        let fps: u32 = if fps_str.trim().is_empty() {
            24
        } else {
            fps_str.trim().parse().unwrap_or(24)
        };
        println!("→ FPS set to {}", fps);

        // 3) Output format
        println!("\nStep 3: Output format");
        println!("  [1] WebM (VP9, can keep transparency)");
        println!("  [2] MP4 (H.264, no alpha)");
        println!("  [3] GIF (8-bit)");

        let format_choice = loop {
            let choice = prompt("Choose format (1/2/3, default 1): ");
            let trimmed = choice.trim();
            if trimmed.is_empty() || trimmed == "1" {
                break "webm".to_string();
            } else if trimmed == "2" {
                break "mp4".to_string();
            } else if trimmed == "3" {
                break "gif".to_string();
            } else {
                println!("Invalid choice, please type 1, 2, or 3.");
            }
        };

        // 4) Transparency
        let mut transparent = false;
        if format_choice == "webm" || format_choice == "gif" {
            let ans = prompt("Keep transparency (alpha)? (y/n, default y): ");
            let a = ans.to_lowercase();
            transparent = a.is_empty() || a.starts_with('y');
            println!("→ Transparency: {}", if transparent { "ON" } else { "OFF" });
        } else {
            println!("→ Transparency ignored for MP4 (no alpha support).");
        }

        // 5) Crop
        println!("\nStep 4: Crop (optional)");
        let crop_ans = prompt("Apply crop? (y/n, default n): ");
        let crop_ans_l = crop_ans.to_lowercase();
        let mut crop_string: Option<String> = None;

        if crop_ans_l.starts_with('y') {
            let w_str = prompt("  Enter crop width (w): ");
            let h_str = prompt("  Enter crop height (h): ");
            let x_str = prompt("  Enter crop x offset: ");
            let y_str = prompt("  Enter crop y offset: ");

            let w: i32 = w_str.trim().parse().unwrap_or(640);
            let h: i32 = h_str.trim().parse().unwrap_or(480);
            let x: i32 = x_str.trim().parse().unwrap_or(0);
            let y: i32 = y_str.trim().parse().unwrap_or(0);

            crop_string = Some(format!("{}:{}:{}:{}", w, h, x, y));
            println!("→ Crop set to: {}", crop_string.as_ref().unwrap());
        } else {
            println!("→ No crop will be applied.");
        }

        // 6) Output file name
        println!("\nStep 5: Output file name");
        let default_ext = &format_choice;
        let default_output = format!("output.{}", default_ext);
        let out_input = prompt(&format!(
            "Enter output file name (default '{}'): ",
            default_output
        ));
        let output_file = if out_input.trim().is_empty() {
            default_output
        } else {
            out_input.trim().to_string()
        };
        println!("→ Output file: {}", output_file);

        // 7) Show summary & confirm
        println!("\n=== Summary ===");
        println!("Input pattern : {}", input_pattern);
        println!("FPS           : {}", fps);
        println!("Format        : {}", format_choice.to_uppercase());
        println!("Transparency  : {}", if transparent { "ON" } else { "OFF" });
        println!("Crop          : {}", crop_string.as_deref().unwrap_or("None"));
        println!("Output file   : {}", output_file);

        let confirm = prompt("\nProceed with these settings? (y/n, default y): ");
        let confirm_l = confirm.to_lowercase();
        if !confirm_l.is_empty() && !confirm_l.starts_with('y') {
            println!("Cancelled by user. Starting over...");
            continue;
        }

        // 8) Build ffmpeg args
        let mut ff_args: Vec<String> = Vec::new();

        // Common
        ff_args.push("-framerate".into());
        ff_args.push(fps.to_string());
        ff_args.push("-i".into());
        ff_args.push(input_pattern.clone());

        // Format-specific
        if format_choice == "webm" {
            build_webm_args(&mut ff_args, &crop_string, transparent);
        } else if format_choice == "mp4" {
            build_mp4_args(&mut ff_args, &crop_string);
        } else {
            build_gif_args(&mut ff_args, &crop_string);
        }

        // Output file
        ff_args.push(output_file.clone());

        // Show the ffmpeg command (for debugging / copy-paste)
        println!("\nRunning ffmpeg command:");
        println!("ffmpeg {}", ff_args.join(" "));

        // 9) Run ffmpeg
let status = Command::new("ffmpeg")
    .args(&ff_args)
    .status();

match status {
    Ok(status) => {
        if status.success() {
            println!("\n✅ Conversion finished successfully!");

            // 10) Show metadata using ffmpeg -i (info only)
            println!("\n=== Output file metadata (ffmpeg -i) ===");
            let _ = Command::new("ffmpeg")
                .args(&["-i", &output_file, "-hide_banner"])
                .status();
        } else {
            eprintln!("\n❌ ffmpeg exited with code {:?}", status.code());
        }
    }
    Err(e) => {
        eprintln!("\n❌ Failed to start ffmpeg: {e}");
        eprintln!("Make sure 'ffmpeg' is installed and visible in your PATH,");
        eprintln!("or put the ffmpeg executable in the same folder as this program.");
        eprintln!("Download: https://ffmpeg.org/download.html");
    }
}

    }
}

/// Prompt the user and read a single line (trimmed).
fn prompt(message: &str) -> String {
    print!("{}", message);
    let _ = io::stdout().flush();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    input.trim_end().to_string()
}

/// Build ffmpeg args for WebM (VP9).
fn build_webm_args(ff_args: &mut Vec<String>, crop: &Option<String>, transparent: bool) {
    // If we want transparency or crop, use filter_complex pipeline.
    if transparent || crop.is_some() {
        let mut filters: Vec<String> = Vec::new();
        // Work in RGBA to keep alpha clean
        filters.push("format=rgba".into());

        if let Some(c) = crop {
            filters.push(format!("crop={}", c));
        }

        let filter_str = format!("[0:v]{}[v]", filters.join(","));
        ff_args.push("-filter_complex".into());
        ff_args.push(filter_str);
        ff_args.push("-map".into());
        ff_args.push("[v]".into());
    } else if let Some(c) = crop {
        // Crop without transparency
        let filter_str = format!("[0:v]crop={}[v]", c);
        ff_args.push("-filter_complex".into());
        ff_args.push(filter_str);
        ff_args.push("-map".into());
        ff_args.push("[v]".into());
    }

    ff_args.push("-c:v".into());
    ff_args.push("libvpx-vp9".into());

    ff_args.push("-pix_fmt".into());
    if transparent {
        ff_args.push("yuva420p".into()); // VP9 + alpha
    } else {
        ff_args.push("yuv420p".into());
    }

    ff_args.push("-auto-alt-ref".into());
    ff_args.push("0".into());
}

/// Build ffmpeg args for MP4 (H.264, no alpha).
fn build_mp4_args(ff_args: &mut Vec<String>, crop: &Option<String>) {
    if let Some(c) = crop {
        let filter_str = format!("[0:v]crop={}[v]", c);
        ff_args.push("-filter_complex".into());
        ff_args.push(filter_str);
        ff_args.push("-map".into());
        ff_args.push("[v]".into());
    }

    ff_args.push("-c:v".into());
    ff_args.push("libx264".into());
    ff_args.push("-pix_fmt".into());
    ff_args.push("yuv420p".into());
}

/// Build ffmpeg args for GIF (palettegen + paletteuse).
fn build_gif_args(ff_args: &mut Vec<String>, crop: &Option<String>) {
    let filter_complex = if let Some(c) = crop {
        format!(
            "[0:v]format=rgba,crop={c}[tmp];\
             [tmp]split[v0][v1];\
             [v0]palettegen[p];\
             [v1][p]paletteuse"
        )
    } else {
        "[0:v]split[v0][v1];[v0]palettegen[p];[v1][p]paletteuse".to_string()
    };

    ff_args.push("-filter_complex".into());
    ff_args.push(filter_complex);
}
