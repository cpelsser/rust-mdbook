-- Pandoc Lua filter to remove <details> blocks (speaker notes) from PDF output
-- Processes the document linearly to correctly handle <details>...</details> sections

function Pandoc(doc)
  local new_blocks = {}
  local in_details = false

  for _, block in ipairs(doc.blocks) do
    -- Check for opening <details> tag
    if block.t == "RawBlock" and block.format == "html" then
      if block.text:match("<details") then
        in_details = true
      elseif block.text:match("</details") then
        in_details = false
      elseif not in_details then
        table.insert(new_blocks, block)
      end
    elseif not in_details then
      table.insert(new_blocks, block)
    end
  end

  return pandoc.Pandoc(new_blocks, doc.meta)
end
