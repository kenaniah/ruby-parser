res = <<~MARK \
foobar
MARK
"meh"  "Blah"
puts res

puts "----"

puts <<~STUFF
  #{<<-foo * 2} bar
  meh
  foo
STUFF

puts "----"

puts <<~INDENT
    foo
    bar#{
2+8
} stuff
  3
INDENT
