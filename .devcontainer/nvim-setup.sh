#!/bin/bash
set -e

echo "[neovim] Instalando Neovim nightly..."

# Add Neovim unstable PPA
sudo apt-get update
sudo apt-get install -y software-properties-common
sudo add-apt-repository -y ppa:neovim-ppa/unstable
sudo apt-get update
sudo apt-get install -y neovim

echo "[neovim] Installing lazy.nvim and Rust tooling config..."

NVIM_CONFIG="$HOME/.config/nvim"
mkdir -p "$NVIM_CONFIG"

# init.lua with Rust tooling
cat > "$NVIM_CONFIG/init.lua" <<'EOF'
-- Bootstrap lazy.nvim
local lazypath = vim.fn.stdpath("data") .. "/lazy/lazy.nvim"
if not vim.loop.fs_stat(lazypath) then
  vim.fn.system({
    "git",
    "clone",
    "--filter=blob:none",
    "https://github.com/folke/lazy.nvim.git",
    "--branch=stable",
    lazypath,
  })
end
vim.opt.rtp:prepend(lazypath)

-- Install plugins
require("lazy").setup({
  { "folke/tokyonight.nvim" },
  { "nvim-lualine/lualine.nvim" },
  {
    "nvim-treesitter/nvim-treesitter",
    build = ":TSUpdate",
  },
  -- LSP
  { "neovim/nvim-lspconfig" },
  -- Completion
  { "hrsh7th/nvim-cmp" },
  { "hrsh7th/cmp-nvim-lsp" },
  { "L3MON4D3/LuaSnip" },
  { "saadparwaiz1/cmp_luasnip" },
})

-- Options
vim.opt.number = true
vim.opt.relativenumber = true
vim.cmd.colorscheme("tokyonight")

-- LSP Setup
local lspconfig = require("lspconfig")
local capabilities = require("cmp_nvim_lsp").default_capabilities()

-- Enable rust-analyzer
lspconfig.rust_analyzer.setup({
  capabilities = capabilities,
})

-- Completion setup
local cmp = require("cmp")
cmp.setup({
  snippet = {
    expand = function(args)
      require("luasnip").lsp_expand(args.body)
    end,
  },
  mapping = cmp.mapping.preset.insert({
    ["<Tab>"] = cmp.mapping.select_next_item(),
    ["<S-Tab>"] = cmp.mapping.select_prev_item(),
    ["<CR>"] = cmp.mapping.confirm({ select = true }),
  }),
  sources = {
    { name = "nvim_lsp" },
    { name = "luasnip" },
  },
})

-- Lualine
require("lualine").setup()
EOF

echo "[neovim] Rust tooling config installed."

