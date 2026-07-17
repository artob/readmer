abort("Expected Ruby 3.4+, but got #{RUBY_VERSION}.") if RUBY_VERSION < '3.4.0'

READMER_SUBCOMMANDS = %w[describe render]

task default: %w[codegen]

desc "Generate .config/readmer/*.sh-session files"
task codegen: %w[.config/readmer/readmer.sh-session] +
  READMER_SUBCOMMANDS.map { ".config/readmer/readmer-#{it}.sh-session" }.to_a

([nil] + READMER_SUBCOMMANDS).each do |subcommand|
  command = subcommand ? "readmer #{subcommand} --help" : "readmer"
  filename = command.delete_suffix(' --help').gsub(' ', '-')
  desc "Generate .config/readmer/#{filename}.sh-session"
  file ".config/readmer/#{filename}.sh-session" do |t|
    File.open(t.name, 'w') do |f|
      f.puts "$ #{command}"
      f.puts `#{command}`
    end
  end
end
