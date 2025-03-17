<div align="center">
> [!TIP]
> Major Release 1.2 Is out! 🎉
</div>

# wrenchlib

<div align="center">

A library for creating fast system configuration utilities written in Rust 🦀

</div>

## Features

- **Minimal**: The library doesn't have any external depencices. (I will try to keep it that way)
- **Custom Git clone function**: Custom git function in case you need it to install dotfiles etc.
- **Use alternative pkg managers!**: The library can install package managers for Windows
- **Manage Scoop Buckets**: You can add scoop buckets, the library includes a custom type
- **Install Packages**: Automatically detects platform and package manager.
- **Networking**: Has helper functions to download files through various methods.
- **Cross-Platform Support**: Compatiable with Linux, Windows, and eventually MacOS
- **Flexible API Frontend**: Simple and easy-to-use API for developers to integrate into their own tools.
- **Distro-Agnostic**: Automatically detects your package manager and distro on Linux

## Usage

### Installing Packages

You can install packages on Linux or Windows using the `install_package` function. It automatically detects the platform and uses the appropriate package manager.

#### Example:

```rust
use wrenchlib::tasks::install::install_package;

fn main() {
    let package_name = "ollama";

    match install_package(package_name, None) { // Easy front-end, works both Windows and Linux without worrying about
        Ok(_) => println!("Successfully installed the package!"),
        Err(e) => eprintln!("Failed to install the package: {}", e),
    }
}
```

## Don't want to use the default package manager?

```rust
use wrenchlib::tasks::install::install_package;

fn main() {
    let package_name = "obs-studio";
                                        //Select your favorite pkg mgr!
    match install_package(package_name, Some(wrenchlib::os::WinPkgMgrs::Scoop)) {
        Ok(_) => println!("Successfully installed the package!"),
        Err(e) => eprintln!("Failed to install the package: {}", e),
    }
}
```

# Managing scoop buckets on Windows

```rust
use wrenchlib::os::windows::scoop_add_bucket;
use wrenchlib::os::windows::ScoopBucket;
fn main() {
    match scoop_add_bucket(ScoopBucket::Extras) {
        Ok(_) => println!("Successfully added bucket."),
        Err(e) => panic!("Failed to add bucket: {}", e),
    }
}
```

Use the custom type to select which bucket to add! **(See it yourself in [windows.rs](/src/os/windows.rs))**

```rust
pub enum ScoopBucket {
    Main,
    Extras,
    Versions,
    Nirsoft,
    Sysinternals,
    Php,
    NerdFonts,
    Nonportable,
    Java,
    Games,
}
```

---

## Supported Platforms

- **Linux**: Supports all major Linux distributions with package managers like `apt`, `pacman`, `dnf`, etc.
- **Windows**: Defaults to Winget but supports Scoop and Chocolatey too.

---

## Contributing

We welcome contributions to improve this library! If you'd like to contribute, please follow these steps:

1.  Fork the repository.
2.  Create a new branch (`git checkout -b feature-name`).
3.  Make your changes and commit (`git commit -am 'Add new feature'`).
4.  Push to your branch (`git push origin feature-name`).
5.  Create a new Pull Request.

---

## License

This project is licensed under the MIT License - see the LICENSE file for details.

---

## Additional Notes

- Ensure that the appropriate package manager or privilege tool (e.g., `sudo`, `doas`, or `winget`) is installed on the system before using the library.
- On Windows, make sure you have `winget` installed and properly configured.
- If any package manager is not installed, Wrenchlib will prompt you for permission to install it.

### Troubleshooting

If you encounter issues with package installation or privilege escalation, ensure the following:

1.  **Linux**: Make sure you have a supported package manager installed (`apt`, `pacman`, etc.).
2.  **Windows**: Ensure `winget` is installed and configured.
3.  **Privilege Tool**: On Linux, ensure `sudo` or `doas` is available for running commands as root.

### Privileged Commands

Some commands in this library require administrative privileges to execute. Ensure you:

- Run the program with `sudo` on Linux for tasks like package installation.
- Use "Run as Administrator" on Windows for elevated commands.
  The library will notify you when elevated privileges are necessary.

### Privilege Escalation

This library supports both `sudo` and `doas` for executing commands with elevated privileges. Ensure that:

- `sudo` is installed and configured correctly, or
- `doas` is installed with the necessary permissions configured in `/etc/doas.conf`.

---

## Roadmap

Future features planned for this library:

- Enhanced error handling for failed package installations.
- Support for more platforms (e.g., macOS).
- Additional OS automation tools (e.g., service management, debloating utilities).
- Integration with popular configuration management tools like Ansible or Puppet.

## Version & Branches

| Branch      | Version |                  |
| ----------- | ------- | ---------------- |
| Stable      | 0.0.0   | [In Development] |
| Development | 0.2.0   |                  |

---

## Part of Infinite's Code Charity 2025 Event

![CharityBanner](docs/assets/BannerCharity.jpg)

# Join and Contribute!
