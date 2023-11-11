# Edit this configuration file to define what should be installed on
# your system.  Help is available in the configuration.nix(5) man page
# and in the NixOS manual (accessible by running ‘nixos-help’).

{ config, pkgs, ... }:

{
  imports =
    [ # Include the results of the hardware scan.
      ./hardware-configuration.nix
    ];

  # Bootloader
  boot.loader = {
    grub = {
      enable = true;
      devices = [ "nodev" ];
      efiSupport = true;
      useOSProber = true;
    };

    efi = {
      canTouchEfiVariables = true;
      efiSysMountPoint = "/boot";
    };
  };
  
  boot.supportedFilesystems = [ "ext4" "ntfs" ];

  # Unlock Gata
  boot.initrd.luks.devices = {
    Gata = {
      device = "/dev/nvme0n1p3"; # replace with your actual partition
      preLVM = false; # set to true if the device is a physical partition and not a logical volume
    };
  };
  
  # Mount Gata
  fileSystems."/Gata" =
  { device = "/dev/mapper/Gata";
    fsType = "ext4";
    options = [ "nosuid" "nodev" "nofail" "x-gvfs-show" ];
  };

  # Mount Boot
  # fileSystems."/boot" =
  # { device = "UUID=EB7E-8D74";
  #   fsType = "vfat";
  # };

  # Mount Swap Memory
  swapDevices = [ { device = "/dev/nvme0n1p6"; } ];

  networking.hostName = "greenix"; # Define your hostname.
  # networking.wireless.enable = true;  # Enables wireless support via wpa_supplicant.

  # Configure network proxy if necessary
  # networking.proxy.default = "http://user:password@proxy:port/";
  # networking.proxy.noProxy = "127.0.0.1,localhost,internal.domain";

  # Enable networking
  networking.networkmanager.enable = true;

  # Set your time zone.
  time.timeZone = "Australia/Melbourne";

  # Select internationalisation properties.
  i18n.defaultLocale = "en_GB.UTF-8";

  i18n.extraLocaleSettings = {
    LC_ADDRESS = "en_AU.UTF-8";
    LC_IDENTIFICATION = "en_AU.UTF-8";
    LC_MEASUREMENT = "en_AU.UTF-8";
    LC_MONETARY = "en_AU.UTF-8";
    LC_NAME = "en_AU.UTF-8";
    LC_NUMERIC = "en_AU.UTF-8";
    LC_PAPER = "en_AU.UTF-8";
    LC_TELEPHONE = "en_AU.UTF-8";
    LC_TIME = "en_AU.UTF-8";
  };

  # Enable the X11 windowing system.
  services.xserver.enable = true;

  # Enable the KDE Plasma Desktop Environment.
  services.xserver.displayManager.sddm.enable = true;
  services.xserver.desktopManager.plasma5.enable = true;


  # Configure keymap in X11
  services.xserver = {
    layout = "us";
    xkbVariant = "";
  };

  # Enable CUPS to print documents.
  services.printing.enable = true;

  # Enable sound with pipewire.
  sound.enable = true;
  hardware.pulseaudio.enable = false;
  security.rtkit.enable = true;
  services.pipewire = {
    enable = true;
    alsa.enable = true;
    alsa.support32Bit = true;
    pulse.enable = true;
    # If you want to use JACK applications, uncomment this
    # jack.enable = true;

    # use the example session manager (no others are packaged yet so this is enabled by default,
    # no need to redefine it in your config for now)
    # media-session.enable = true;
  };

  # Enable touchpad support (enabled default in most desktopManager).
  services.xserver.libinput.enable = true;

  # Enable automatic login
  services.xserver.displayManager.autoLogin.enable = true;
  services.xserver.displayManager.autoLogin.user = "greenchild";

  # set zsh as default shell
  users.defaultUserShell = pkgs.zsh;

  # Define a user account. Don't forget to set a password with ‘passwd’.
  users.users.greenchild = {
    isNormalUser = true;
    description = "GreenChild";
    extraGroups = [ "networkmanager" "wheel" ];
  };

  # Allow unfree packages
  nixpkgs.config.allowUnfree = true;

  # Enable docker
  virtualisation.docker.enable = true;

  # List packages installed in system profile. To search, run:
  # $ nix search wget
  environment.systemPackages = with pkgs; [
    # [ Overrides ]
    # Use an older version of vscodium cause the new one doesn't work
    (pkgs.vscodium.overrideAttrs (oldAttrs: rec {
      src = fetchurl {
        url = "https://github.com/VSCodium/vscodium/releases/download/${version}/VSCodium-linux-x64-${version}.tar.gz";
        sha256 = "a606e540f8dfe5a049513138adb01f03d6005cbb9b1b6a871688462ea51aa573";
      };
      version = "1.81.1.23222";
    }))

    # [ Programming Langs and Libs ]
    pipx
    libclang
    rustup
    python311
    python311Packages.pip
    dotnet-sdk
    gcc

    # [ Programming Apps ]
    git
    bacon
    github-desktop
    cargo-ui
    vscodium
    jetbrains.clion
    cargo-expand

    # [ Browsers ]
    microsoft-edge
    brave
    firefox # for public wifi gateway reasons

    # [ Text Editors ]
    helix
    kate
    neovim

    # [ TUI ]
    xplr # file explorer
    tmux
    
    # [ CLI ]
    docker
    mdcat
    taplo # toml reader
    wget
    # curl
    neofetch
    wineWowPackages.minimal
    pandoc
    protonvpn-cli_2
    ffmpeg

    # [ GUI ]
    qbittorrent
    # libsForQt5.ghostwriter
    gparted
    obsidian
    krita
    audacity
    discord
    vlc # video player
    libsForQt5.kdenlive # video editor
    typora # markdown

    # [ Dependencies ]
    gcc-arm-embedded-7 # for robotics
    thefuck # for zsh (console intellisense)
    exfatprogs # for gparted
    sccache # for speeding up rust compile time
    mediainfo # for kdenlive
    x264 # for kdenlive
    texlive.combined.scheme-small # for pandoc
    hunspell hunspellDicts.uk_UA hunspellDicts.th_TH # for libreoffice spell check

    # [ Critical stuff ]
    gnupg
    zsh
    ntfs3g # support for ntfs mounting
    cryptsetup # for encryption
    polkit
    os-prober
    # bluez # for blutooth (not working :C)

    # [ Power saving ]
    powertop
    tlp
    
    # [ School ]
    libreoffice-fresh
  ];

  # Some programs need SUID wrappers, can be configured further or are
  # started in user sessions.
  # programs.mtr.enable = true;
  programs.gnupg.agent = {
    enable = true;
    enableSSHSupport = true;
  };

  # List services that you want to enable:

  # Enable the OpenSSH daemon.
  services.openssh.enable = true;

  # Open ports in the firewall.
  # networking.firewall.allowedTCPPorts = [ ... ];
  # networking.firewall.allowedUDPPorts = [ ... ];
  # Or disable the firewall altogether.
  # networking.firewall.enable = false;

  # This value determines the NixOS release from which the default
  # settings for stateful data, like file locations and database versions
  # on your system were taken. It‘s perfectly fine and recommended to leave
  # this value at the release version of the first install of this system.
  # Before changing this value read the documentation for this option
  # (e.g. man configuration.nix or on https://nixos.org/nixos/options.html).
  system.stateVersion = "23.05"; # Did you read the comment?

  # Enable flakes and nix command
  nix.settings.experimental-features = [ "nix-command" "flakes" ];
  
  # [ Zsh ]
  programs.zsh = {
    enable = true;
    shellAliases = {
      ll = "ls -l";
      clean = "nix-store --gc";
      full-clean = "sudo nix-collect-garbage -d";
      axolotl = "dotnet /Gata/Programs/Axolotl/Axolotl.dll /Gata/Ethan/Axolotl";
      configure = "sudo rm /etc/nixos/old_configuration.nix; sudo cp /etc/nixos/configuration.nix /etc/nixos/old_configuration.nix; sudo hx /etc/nixos/configuration.nix; sudo nixos-rebuild switch";
      configs = "cd /Gata/Ethan/Home/Github/personal-configs";
      claer = "clear";
      potato-farm = "/Gata/Programs/potato-farm";
      thoughts = "/Gata/Programs/thoughts";
      diary-cli = "/Gata/Ethan/Home/Github/diary-cli/target/release/diary-cli";
      gres = "/Gata/Programs/gres";
      onefig = "/Gata/Programs/onefig";
      nf = "/Gata/Programs/onefig";
      fix-ntfs = "sudo ntfsfix -d";
    };
  };

  # [ Git ]
  programs.git.enable = true;

  # [ Power Saving ]
  services.tlp.enable = true;
  services.power-profiles-daemon.enable = false;

  # [ Etc ]
  environment.etc."mdadm.conf".text = ''
    MAILADDR greenchild04@protonmail.com
  '';

  # [ Env Varibles ]
  environment.variables = {
    LIBCLANG_PATH = "/nix/store/2l475hynw6hmxn81m3m4ka231z22kvk2-clang-11.1.0-lib/lib/libclang.so";
    OPENSSL_DIR = "/nix/store/y2hmc1ypa5yw54jsizxbn9gjag0d468k-openssl-3.0.10";
  };
}
