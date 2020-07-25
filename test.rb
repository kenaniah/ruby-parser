res = <<~MARK \
foobar
MARK
"meh"  "Blah"
puts res

puts  "----"

puts <<~STUFF
  #{<<-foo * 2} bar
  meh
  foo
STUFF
