#!/bin/bash
#
# DragonCore Runtime Launch Script
# 龙核运行时启动脚本
#

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Configuration
DRAGONCORE_BIN="${SCRIPT_DIR}/target/release/dragoncore-runtime"
DATA_DIR="${PROJECT_ROOT}/data"
CONFIG_FILE="${PROJECT_ROOT}/dragoncore.toml"

# Print banner
print_banner() {
    echo -e "${BLUE}"
    echo "╔════════════════════════════════════════════════════════════╗"
    echo "║                                                            ║"
    echo "║   DragonCore 龙核                                          ║"
    echo "║   Governance-First Multi-Agent AI Operating System         ║"
    echo "║                                                            ║"
    echo "║   真龙，不是龙虾 | True Dragon, Not Claw                  ║"
    echo "║                                                            ║"
    echo "╚════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

# Check dependencies
check_dependencies() {
    echo -e "${BLUE}Checking dependencies...${NC}"
    
    # Check tmux
    if ! command -v tmux &> /dev/null; then
        echo -e "${RED}Error: tmux is not installed${NC}"
        echo "Please install tmux: sudo apt-get install tmux"
        exit 1
    fi
    
    # Check git
    if ! command -v git &> /dev/null; then
        echo -e "${RED}Error: git is not installed${NC}"
        echo "Please install git: sudo apt-get install git"
        exit 1
    fi
    
    echo -e "${GREEN}✓ All dependencies satisfied${NC}"
}

# Check if binary exists
check_binary() {
    if [ ! -f "$DRAGONCORE_BIN" ]; then
        echo -e "${YELLOW}DragonCore binary not found, building...${NC}"
        cd "$SCRIPT_DIR"
        cargo build --release
        
        if [ ! -f "$DRAGONCORE_BIN" ]; then
            echo -e "${RED}Error: Failed to build DragonCore binary${NC}"
            exit 1
        fi
    fi
    
    echo -e "${GREEN}✓ DragonCore binary ready${NC}"
}

# Initialize configuration
init_config() {
    if [ ! -f "$CONFIG_FILE" ]; then
        echo -e "${YELLOW}Initializing DragonCore configuration...${NC}"
        mkdir -p "$DATA_DIR"
        "$DRAGONCORE_BIN" --config "$CONFIG_FILE" init --output "$PROJECT_ROOT"
        echo -e "${GREEN}✓ Configuration initialized at $CONFIG_FILE${NC}"
    fi
}

# Show help
show_help() {
    echo "Usage: $0 [COMMAND] [OPTIONS]"
    echo ""
    echo "Commands:"
    echo "  run [TASK]       Start a new governance run"
    echo "  status           Show status of active runs"
    echo "  metrics          Show stability metrics"
    echo "  seats            List all 19 governance seats"
    echo "  attach [RUN_ID]  Attach to tmux session for a run"
    echo "  cleanup          Clean up all resources"
    echo "  build            Build the DragonCore binary"
    echo "  help             Show this help message"
    echo ""
    echo "Environment Variables:"
    echo "  KIMI_API_KEY      API key for Kimi (Moonshot AI)"
    echo "  DEEPSEEK_API_KEY  API key for DeepSeek"
    echo "  QWEN_API_KEY      API key for Qwen (Alibaba)"
    echo ""
    echo "Examples:"
    echo "  $0 run \"Implement a new feature\""
    echo "  $0 status"
    echo "  $0 attach RUN-20240314-001"
}

# Main command handler
case "${1:-run}" in
    run)
        print_banner
        check_dependencies
        check_binary
        init_config
        
        TASK="${2:-General governance run}"
        echo -e "${BLUE}Starting governance run...${NC}"
        "$DRAGONCORE_BIN" --config "$CONFIG_FILE" run --task "$TASK"
        ;;
    
    status)
        print_banner
        check_binary
        "$DRAGONCORE_BIN" --config "$CONFIG_FILE" status
        ;;
    
    metrics)
        print_banner
        check_binary
        "$DRAGONCORE_BIN" --config "$CONFIG_FILE" metrics
        ;;
    
    seats)
        print_banner
        check_binary
        "$DRAGONCORE_BIN" seats
        ;;
    
    attach)
        if [ -z "$2" ]; then
            echo -e "${RED}Error: Run ID required${NC}"
            echo "Usage: $0 attach [RUN_ID]"
            exit 1
        fi
        "$DRAGONCORE_BIN" attach --run-id "$2"
        ;;
    
    cleanup)
        echo -e "${YELLOW}Cleaning up DragonCore resources...${NC}"
        check_binary
        "$DRAGONCORE_BIN" --config "$CONFIG_FILE" cleanup
        echo -e "${GREEN}✓ Cleanup complete${NC}"
        ;;
    
    build)
        print_banner
        echo -e "${BLUE}Building DragonCore Runtime...${NC}"
        cd "$SCRIPT_DIR"
        cargo build --release
        echo -e "${GREEN}✓ Build complete${NC}"
        echo "Binary location: $DRAGONCORE_BIN"
        ls -lh "$DRAGONCORE_BIN"
        ;;
    
    help|--help|-h)
        show_help
        ;;
    
    *)
        echo -e "${RED}Unknown command: $1${NC}"
        show_help
        exit 1
        ;;
esac
