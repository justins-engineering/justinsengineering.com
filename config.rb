# Activate and configure extensions
# https://middlemanapp.com/advanced/configuration/#configuring-extensions

out_dir = './build'

activate :external_pipeline,
         name: :tailwindcss_build,
         command: "bunx @tailwindcss/cli -i ./source/stylesheets/site.css -o #{out_dir}/stylesheets/site.css " \
                  "#{build? ? '--minify' : '--watch'}",
         latency: 2,
         source: out_dir

# activate :directory_indexes

# Development Files

ignore '/stylesheets/fontawesome.css'
ignore '/webfonts/*.sfd'
ignore '/**/.keep'

# Layouts
# https://middlemanapp.com/basics/layouts/

page '/*.xml', layout: false
page '/*.json', layout: false
page '/*.txt', layout: false

# Helpers
# Methods defined in the helpers block are available in templates
# https://middlemanapp.com/basics/helper-methods/

# helpers do
#   def some_helper
#     'Helping'
#   end
# end

# Build-specific configuration
# https://middlemanapp.com/advanced/configuration/#environment-specific-settings

# configure :build do
#   activate :minify_javascript
# end
