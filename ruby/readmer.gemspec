Gem::Specification.new do |gem|
  gem.version            = File.read('VERSION').chomp
  gem.date               = File.mtime('VERSION').strftime('%Y-%m-%d')

  gem.name               = "readmer"
  gem.homepage           = "https://readmer.dev"
  gem.license            = "Unlicense"
  gem.summary            = "Readmer for Ruby"
  gem.description        = "Compose README.md files from templates."
  gem.metadata           = {
    'bug_tracker_uri'   => "https://github.com/artob/readmer/issues",
    'changelog_uri'     => "https://github.com/artob/readmer/blob/master/CHANGES.md",
    'documentation_uri' => "https://rubydoc.info/gems/readmer",
    'homepage_uri'      => "https://readmer.dev",
    'source_code_uri'   => "https://github.com/artob/readmer",
  }

  gem.author             = "Arto Bendiken"
  gem.email              = "arto@bendiken.net"

  gem.platform           = Gem::Platform::RUBY
  gem.files              = %w(AUTHORS CHANGES.md README.md UNLICENSE VERSION) + Dir.glob('lib/**/*.rb')
  gem.bindir             = %q(bin)
  gem.executables        = %w()

  gem.required_ruby_version = '>= 4.0'
  gem.add_development_dependency 'rspec', '~> 3.13'
  gem.add_development_dependency 'yard' , '~> 0.9'
end
