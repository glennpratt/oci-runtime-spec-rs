#!/usr/bin/env ruby

filepath = 'src/spec.rs'
text = File.read(filepath)

# Horrifying hack to add Default to structs, except the ones containing enums with no default.
re = /Deserialize(\)\]\n[\w ]+struct (?!(NamespaceReference|Seccomp|Syscall|SyscallArg|Image|WindowsDevice|StateSchema))[\w]+ \{)/m
text = text.gsub(re, 'Deserialize, Default, PartialEq\1')

# Add PartialEq to everything.
re = /Deserialize(\)\]\n[\w ]+(?:enum|struct) [\w]+ \{)/m
text = text.gsub(re, 'Deserialize, PartialEq\1')

File.open(filepath, "w") {|file| file.puts(text)}
