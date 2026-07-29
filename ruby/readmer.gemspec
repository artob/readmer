# See: https://docs.ruby-lang.org/en/4.0/Gem/Specification.html

require 'distrib/ruby/gemspec'

Distrib::Ruby::Gemspec.build!(__FILE__) do |gemspec|
  gemspec.summary     = "Readmer for Ruby"
  gemspec.description = "Compose README.md files from templates."
  gemspec.homepage    = "https://readmer.dev"
  gemspec.metadata    = {
    :source_code_uri => "https://github.com/artob/readmer",
    :bug_tracker_uri => "https://github.com/artob/readmer/issues",
    :changelog_uri   => "https://github.com/artob/readmer/blob/master/CHANGES.md",
  }.transform_keys(&:to_s)
end
