function clone
   set into (dolly $argv)
   git clone "$argv" "$into"
   cd "$into"
end
