#!/bin/bash

# =============================
# CONFIG
# =============================

DEBUG=false
USER_NAME="$USER"

# =============================
# COLORES
# =============================

RED="\e[31m"
GREEN="\e[32m"
YELLOW="\e[33m"
BLUE="\e[34m"
CYAN="\e[36m"
RESET="\e[0m"

# =============================
# FUNCIONES
# =============================

log() {
    echo -e "\n${CYAN}[$(date +%H:%M:%S)]${RESET} $1"
}

progress() {
    echo -ne "${YELLOW}➜ $1...${RESET}"
}

success() {
    echo -e " ${GREEN}[OK]${RESET}"
}

fail() {
    echo -e " ${RED}[FAIL]${RESET}"
}

run() {
    progress "$2"

    if $DEBUG; then
        eval "$1"
    else
        eval "$1" > /dev/null 2>&1
    fi

    if [ $? -eq 0 ]; then
        success
    else
        fail
    fi
}

bar() {
    local duration=$1
    for ((i=1;i<=duration;i++)); do
        printf "\r${BLUE}["
        for ((j=1;j<=i;j++)); do printf "▇"; done
        for ((j=i;j<duration;j++)); do printf " "; done
        printf "]${RESET}"
        sleep 0.02
    done
    printf "\n"
}

# =============================
# INICIO
# =============================

clear
echo -e "${GREEN}"
echo "======================================"
echo "        PUNKWM INSTALLER v1.0        "
echo "======================================"
echo -e "${RESET}"

# =============================
# INSTALACIÓN BASE
# =============================

log "Instalando paquetes base"
bar 40

run "sudo pacman -Sy --noconfirm \
xorg-xinit webkit2gtk gtk3 atk pango gdk-pixbuf2 xorg-server \
seatd xorg-xrandr xorg-xsetroot xf86-video-intel xf86-video-amdgpu \
xf86-video-vesa xorg-server-xorgxrdp xorg-xauth feh \
rustup llvm clang sudo pam nano networkmanager firefox zsh unzip wget git" \
"Instalando paquetes principales"

# =============================
# SERVICIOS
# =============================

log "Configurando servicios"

run "sudo systemctl enable --now seatd" "Activando seatd"
run "sudo systemctl enable --now NetworkManager" "Activando NetworkManager"
run "sudo systemctl enable mpd.service acpid.service wpa_supplicant" "Habilitando servicios adicionales"
run "sudo systemctl start mpd.service acpid.service wpa_supplicant" "Iniciando servicios adicionales"

# =============================
# SHELL
# =============================

log "Configurando ZSH"

run "sudo usermod -s /bin/zsh $USER_NAME" "Cambiando shell usuario"
run "sudo usermod -s /bin/zsh root" "Cambiando shell root"

# =============================
# RUST
# =============================

log "Configurando Rust"
run "rustup default stable" "Seteando Rust stable"

# =============================
# SUDOERS
# =============================

log "Configurando permisos especiales"

run "echo 'ALL ALL=NOPASSWD: /usr/bin/nmcli -s -g 802-11-wireless-security.psk connection show *' | sudo tee /etc/sudoers.d/nmcli-psk" \
"Configurando nmcli sudo"

run "echo 'ALL ALL=(ALL) NOPASSWD: /usr/bin/systemctl poweroff, /usr/bin/systemctl reboot' | sudo tee /etc/sudoers.d/nopasswd-shutdown" \
"Configurando apagado sin contraseña"

# =============================
# PARU (AUR)
# =============================

log "Instalando PARU"

if [ ! -d "$HOME/paru-bin" ]; then
    run "git clone https://aur.archlinux.org/paru-bin.git $HOME/paru-bin" "Clonando paru-bin"
fi

cd "$HOME/paru-bin"
run "makepkg -si --noconfirm" "Compilando paru"

# =============================
# PAQUETES AUR
# =============================

log "Instalando paquetes AUR"
run "paru -Sy --noconfirm kitty neofetch xclip lsd bat \
zsh-syntax-highlighting zsh-autosuggestions \
ttf-jetbrains-mono-nerd ttf-font-awesome ttf-font-awesome-4 ttf-material-design-icons" \
"Instalando paquetes AUR"

# =============================
# POWERLEVEL10K
# =============================

log "Instalando Powerlevel10k"

run "git clone --depth=1 https://github.com/romkatv/powerlevel10k.git $HOME/powerlevel10k" \
"Clonando para usuario"

run "sudo git clone --depth=1 https://github.com/romkatv/powerlevel10k.git /root/powerlevel10k" \
"Clonando para root"

# =============================
# CONFIGURACIONES USER
# =============================

log "Copiando configuraciones"

run "cp user/.p10k.zsh $HOME/" "Copiando p10k usuario"
run "sudo cp root/.p10k.zsh /root/" "Copiando p10k root"

run "cp user/.profile $HOME/" "Copiando profile"
run "cp user/.Xresources $HOME/" "Copiando Xresources"
run "cp user/.zshrc $HOME/" "Copiando zshrc"

run "sudo ln -sf /home/$USER_NAME/.zshrc /root/.zshrc" "Linkeando zshrc root"

# =============================
# WM SERVICE
# =============================

log "Activando PunkWM"

run "sudo systemctl enable PunkWM.service" "Habilitando PunkWM"
run "sudo systemctl start PunkWM.service" "Iniciando PunkWM"

# =============================
# FINAL
# =============================

echo -e "\n${GREEN}"
echo "======================================"
echo "       INSTALACIÓN COMPLETADA        "
echo "======================================"
echo -e "${RESET}"