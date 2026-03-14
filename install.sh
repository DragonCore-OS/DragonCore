#!/bin/bash
#
# DragonCore 龙核 - Installation Script
# 安装脚本
#
# 真龙，不是龙虾 | True Dragon, Not Claw
#

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m' # No Color

# Emojis
DRAGON="🐉"
CHECK="✅"
WARN="⚠️ "
ERROR="❌"
GEAR="⚙️ "
PACKAGE="📦"
ROCKET="🚀"
SCROLL="📜"
SHIELD="🛡️ "

# Configuration
REPO_URL="https://github.com/DragonCore-OS/DragonCore"
INSTALL_DIR="${HOME}/.dragoncore"
BIN_DIR="${HOME}/.local/bin"
RUNTIME_DIR="${INSTALL_DIR}/runtime"

# Print DragonCore ASCII Banner
print_banner() {
    echo -e "${CYAN}"
    cat << 'EOF'
    ╔═══════════════════════════════════════════════════════════════╗
    ║                                                               ║
    ║     ██████╗ ██████╗  █████╗  ██████╗  ██████╗ ███╗   ██╗      ║
    ║     ██╔══██╗██╔══██╗██╔══██╗██╔════╝ ██╔═══██╗████╗  ██║      ║
    ║     ██║  ██║██████╔╝███████║██║  ███╗██║   ██║██╔██╗ ██║      ║
    ║     ██║  ██║██╔══██╗██╔══██║██║   ██║██║   ██║██║╚██╗██║      ║
    ║     ██████╔╝██║  ██║██║  ██║╚██████╔╝╚██████╔╝██║ ╚████║      ║
    ║     ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝  ╚═════╝ ╚═╝  ╚═══╝      ║
    ║                                                               ║
    ║              龙 核  -  Governance-First AI OS                 ║
    ║                                                               ║
    ║              真龙，不是龙虾 | True Dragon, Not Claw          ║
    ║                                                               ║
    ╚═══════════════════════════════════════════════════════════════╝
EOF
    echo -e "${NC}"
}

# Print step header
print_step() {
    echo -e "\n${BLUE}${GEAR} $1${NC}"
}

# Print success
print_success() {
    echo -e "${GREEN}${CHECK} $1${NC}"
}

# Print warning
print_warn() {
    echo -e "${YELLOW}${WARN} $1${NC}"
}

# Print error
print_error() {
    echo -e "${RED}${ERROR} $1${NC}"
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check dependencies
check_dependencies() {
    print_step "Checking Dependencies | 检查依赖"
    
    local missing_deps=()
    
    if ! command_exists git; then
        missing_deps+=("git")
    fi
    
    if ! command_exists tmux; then
        missing_deps+=("tmux")
    fi
    
    if ! command_exists curl && ! command_exists wget; then
        missing_deps+=("curl or wget")
    fi
    
    if [ ${#missing_deps[@]} -ne 0 ]; then
        print_error "Missing dependencies: ${missing_deps[*]}"
        echo ""
        echo "Please install missing dependencies:"
        echo "  Ubuntu/Debian: sudo apt-get install -y git tmux curl"
        echo "  CentOS/RHEL:   sudo yum install -y git tmux curl"
        echo "  macOS:         brew install git tmux"
        exit 1
    fi
    
    print_success "All dependencies satisfied"
}

# Detect OS
detect_os() {
    print_step "Detecting Platform | 检测平台"
    
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        OS="linux"
        print_success "Detected: Linux"
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        OS="macos"
        print_success "Detected: macOS"
    else
        print_warn "Unknown OS: $OSTYPE"
        print_warn "Linux assumed, but your mileage may vary"
        OS="linux"
    fi
}

# Check Rust installation
check_rust() {
    print_step "Checking Rust Toolchain | 检查 Rust 工具链"
    
    if command_exists rustc && command_exists cargo; then
        RUST_VERSION=$(rustc --version)
        print_success "Rust found: $RUST_VERSION"
        
        # Check minimum version (1.75+)
        RUST_NUMERIC=$(rustc --version | grep -oP '\d+\.\d+' | head -1)
        REQUIRED="1.75"
        
        if [ "$(printf '%s\n' "$REQUIRED" "$RUST_NUMERIC" | sort -V | head -n1)" != "$REQUIRED" ]; then
            print_warn "Rust version $RUST_NUMERIC is older than required $REQUIRED"
            print_warn "Please update: rustup update"
            exit 1
        fi
    else
        print_error "Rust not found"
        echo ""
        echo "DragonCore requires Rust to build from source."
        echo "Install Rust: https://rustup.rs/"
        echo ""
        echo "Quick install:"
        echo '  curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh'
        exit 1
    fi
}

# Create directories
create_directories() {
    print_step "Creating Directories | 创建目录"
    
    mkdir -p "$INSTALL_DIR"
    mkdir -p "$BIN_DIR"
    mkdir -p "${INSTALL_DIR}/data"
    mkdir -p "${INSTALL_DIR}/data/ledger"
    mkdir -p "${INSTALL_DIR}/data/worktrees"
    
    print_success "Directories created at $INSTALL_DIR"
}

# Clone or update repository
clone_repository() {
    print_step "Downloading DragonCore | 下载 DragonCore"
    
    if [ -d "${RUNTIME_DIR}/.git" ]; then
        print_warn "Existing installation found, updating..."
        cd "$RUNTIME_DIR"
        git pull origin main
    else
        if [ -d "$RUNTIME_DIR" ]; then
            print_warn "Removing incomplete installation..."
            rm -rf "$RUNTIME_DIR"
        fi
        
        echo "Cloning from ${REPO_URL}..."
        git clone --depth 1 "$REPO_URL" "$RUNTIME_DIR"
    fi
    
    print_success "Source code ready"
}

# Build DragonCore
build_dragoncore() {
    print_step "Building DragonCore Runtime | 构建 DragonCore 运行时"
    
    cd "$RUNTIME_DIR"
    
    echo "Compiling (this may take a few minutes)..."
    cargo build --release 2>&1 | while read line; do
        echo "  $line"
    done
    
    if [ ! -f "${RUNTIME_DIR}/target/release/dragoncore-runtime" ]; then
        print_error "Build failed - binary not found"
        exit 1
    fi
    
    print_success "Build successful"
    
    # Show binary info
    BINARY_SIZE=$(ls -lh "${RUNTIME_DIR}/target/release/dragoncore-runtime" | awk '{print $5}')
    echo -e "  ${CYAN}Binary size: $BINARY_SIZE${NC}"
}

# Install binary
install_binary() {
    print_step "Installing Binary | 安装二进制"
    
    cp "${RUNTIME_DIR}/target/release/dragoncore-runtime" "${BIN_DIR}/dragoncore"
    chmod +x "${BIN_DIR}/dragoncore"
    
    print_success "Binary installed to ${BIN_DIR}/dragoncore"
}

# Create launcher script
create_launcher() {
    print_step "Creating Launcher | 创建启动器"
    
    cat > "${BIN_DIR}/dragoncore-launch" << EOF
#!/bin/bash
# DragonCore Launcher
# Generated by install.sh

export DRAGONCORE_HOME="${INSTALL_DIR}"
cd "${RUNTIME_DIR}"
./launch.sh "\$@"
EOF
    
    chmod +x "${BIN_DIR}/dragoncore-launch"
    
    print_success "Launcher created: dragoncore-launch"
}

# Setup shell integration
setup_shell() {
    print_step "Configuring Shell | 配置 Shell"
    
    local shell_rc=""
    local current_shell=""
    
    if [ -n "$ZSH_VERSION" ]; then
        current_shell="zsh"
        shell_rc="$HOME/.zshrc"
    elif [ -n "$BASH_VERSION" ]; then
        current_shell="bash"
        shell_rc="$HOME/.bashrc"
    else
        print_warn "Unknown shell, manual PATH setup required"
        return
    fi
    
    # Check if already in PATH
    if [[ ":$PATH:" == *":$BIN_DIR:"* ]]; then
        print_success "\$BIN_DIR already in PATH"
    else
        echo "" >> "$shell_rc"
        echo "# DragonCore Runtime" >> "$shell_rc"
        echo "export PATH=\"\$PATH:$BIN_DIR\"" >> "$shell_rc"
        print_success "Added to $shell_rc"
        print_warn "Please run: source $shell_rc"
    fi
}

# Create default config
create_config() {
    print_step "Creating Configuration | 创建配置"
    
    local config_file="${INSTALL_DIR}/dragoncore.toml"
    
    if [ -f "$config_file" ]; then
        print_warn "Config already exists, preserving"
        return
    fi
    
    cat > "$config_file" << EOF
# DragonCore Configuration
# 龙核配置文件

[runtime]
name = "dragoncore"
version = "0.1.0"
data_dir = "${INSTALL_DIR}/data"
log_level = "info"

[governance]
constitution_path = "${INSTALL_DIR}/data/constitution"
escalation_timeout = 300
strict_mode = true

[execution]
tmux_prefix = "dragoncore"
worktree_base = "${INSTALL_DIR}/data/worktrees"
max_concurrent_agents = 19
isolation_enabled = true

[ledger]
storage_path = "${INSTALL_DIR}/data/ledger"
auto_archive_threshold = 100
retention_days = 365

# Add your API keys below or set as environment variables:
# export KIMI_API_KEY="your-key"
# export DEEPSEEK_API_KEY="your-key"
# export QWEN_API_KEY="your-key"
EOF
    
    print_success "Configuration created at $config_file"
}

# Print post-installation info
print_post_install() {
    echo -e "\n${GREEN}"
    cat << 'EOF'
    ╔═══════════════════════════════════════════════════════════════╗
    ║                                                               ║
    ║              🐉 安装完成 | Installation Complete 🐉          ║
    ║                                                               ║
    ╚═══════════════════════════════════════════════════════════════╝
EOF
    echo -e "${NC}"
    
    echo -e "${CYAN}DragonCore has been installed to:${NC}"
    echo "  Binary:     ${BIN_DIR}/dragoncore"
    echo "  Launcher:   ${BIN_DIR}/dragoncore-launch"
    echo "  Data:       ${INSTALL_DIR}"
    echo "  Config:     ${INSTALL_DIR}/dragoncore.toml"
    echo ""
    
    echo -e "${CYAN}Quick Start | 快速开始:${NC}"
    echo ""
    echo -e "${YELLOW}1. Set up API key (required):${NC}"
    echo "   export KIMI_API_KEY='your-key-here'"
    echo "   # Or edit: ${INSTALL_DIR}/dragoncore.toml"
    echo ""
    echo -e "${YELLOW}2. Reload shell configuration:${NC}"
    echo "   source ~/.bashrc  # or ~/.zshrc"
    echo ""
    echo -e "${YELLOW}3. Verify installation:${NC}"
    echo "   dragoncore --version"
    echo "   dragoncore seats"
    echo ""
    echo -e "${YELLOW}4. Start your first governance run:${NC}"
    echo "   dragoncore-launch run \"Implement a feature\""
    echo ""
    
    echo -e "${CYAN}Documentation | 文档:${NC}"
    echo "  Usage Guide:  ${RUNTIME_DIR}/../docs/USAGE_GUIDE.md"
    echo "  19 Seats:     ${RUNTIME_DIR}/../docs/19_SEATS.md"
    echo "  GitHub:       ${REPO_URL}"
    echo ""
    
    echo -e "${MAGENTA}真龙，不是龙虾 | True Dragon, Not Claw${NC}"
    echo ""
}

# Main installation flow
main() {
    print_banner
    
    echo -e "${CYAN}Welcome to DragonCore Installer | 欢迎使用 DragonCore 安装程序${NC}"
    echo ""
    
    check_dependencies
    detect_os
    check_rust
    create_directories
    clone_repository
    build_dragoncore
    install_binary
    create_launcher
    setup_shell
    create_config
    
    print_post_install
}

# Handle errors
trap 'print_error "Installation failed. Check logs above."' ERR

# Run main
main "$@"
