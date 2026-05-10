-- classify.lua
-- Minimal, platform-safe classifier stub.
-- You can call this from Rust later via mlua/rlua.

local M = {}

function M.classify_claim(claim)
  claim = tostring(claim or ""):lower()

  if claim:find("mkultra") or claim:find("mind control") then
    return "historical_mind_control_context"
  elseif claim:find("vaccine") then
    return "health_misinformation_context"
  else
    return "generic_claim"
  end
end

return M
