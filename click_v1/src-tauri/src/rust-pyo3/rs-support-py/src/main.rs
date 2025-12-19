use std::process::Command;

fn main(){

}
fn get_explorer_path() -> Result<String, std::io::Error> {
    let output = Command::new("powershell")
        .args(&[
            "-Command",
            "(New-Object -ComObject Shell.Application).Windows() | ForEach-Object { $_.Document.Folder.Self.Path }"
        ])
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
   