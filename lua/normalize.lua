-- normalize.lua
-- Simple text normalization stub.

local M = {}

function M.normalize(text)
  text = tostring(text or "")
  -- Strip leading/trailing whitespace and collapse spaces.
  text = text:gsub("^%s+", ""):gsub("%s+$", ""):gsub("%s+", " ")
  return text
end

return M
