set -e

here=$(dirname "$0")
lldb=$(command -v lldb)

# Colors
GREEN="\033[0;32m"
YELLOW="\033[1;33m"
CYAN="\033[0;36m"
RESET="\033[0m"
RED="\033[0;31m"

if [ -z "$lldb" ]; then
  echo "${RED}---- Error: LLDB is not found in your PATH. Please add it to resolve."
  echo "${YELLOW}---- Please install LLDB (e.g., via Homebrew: 'brew install llvm') and ensure it’s added to your PATH."
  exit 1
fi

# Get LLDB version
LLDB_VERSION=$("$lldb" --version | grep -Eo "[0-9]+\.[0-9]+\.[0-9]+")
REQUIRED_VERSION="20.1.7" # You can adjust this to match what works best


# Output
echo -e "${CYAN}---- Using LLDB from: ${GREEN}$lldb${RESET} ${YELLOW}(version $LLDB_VERSION)${RESET}"
echo -e "${YELLOW}---- Recommended: LLDB version ${REQUIRED_VERSION} or newer${RESET}"

script_import_rust="command script import \"$here/formatters/lldb_lookup.py\""
script_import_solana="command script import \"$here/formatters/solana_lookup.py\""
commands_file_rust="$here/formatters/lldb_commands"
commands_file_solana="$here/formatters/solana_commands"

# Call LLDB with the commands added to the argument list
"$lldb" --one-line-before-file "$script_import_rust" --one-line-before-file "$script_import_solana" --source-before-file "$commands_file_rust" --source-before-file "$commands_file_solana" "$@"
