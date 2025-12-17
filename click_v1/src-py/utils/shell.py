import platform
import subprocess


def shutdown():
    system_name = platform.system()
    if system_name == "Windows":
        subprocess.run(["shutdown", "/s", "/t", "0"])
    elif system_name == "Linux" or system_name == "Darwin":  # Darwin 适用于 macOS
        subprocess.run(["sudo", "shutdown", "-h", "now"], check=True)
    else:
        print("此操作系统不支持")


def restart_computer():
    system_name = platform.system()
    if system_name == "Windows":
        subprocess.run(["shutdown", "/r", "/t", "0"])
    elif system_name in ["Linux", "Darwin"]:  # Darwin 是 macOS
        subprocess.run(["sudo", "reboot"])
    else:
        print("此操作系统不支持")
