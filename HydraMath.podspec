#
# Be sure to run `pod lib lint HydraMath.podspec' to ensure this is a
# valid spec before submitting.
#
# Any lines starting with a # are optional, but their use is encouraged
# To learn more about a Podspec see https://guides.cocoapods.org/syntax/podspec.html
#

Pod::Spec.new do |s|
  s.name             = 'HydraMath'
  s.version          = '0.1.0'
  s.summary          = 'A short description of HydraMath.'

# This description is used to generate tags and improve search results.
#   * Think: What does it do? Why did you write it? What is the focus?
#   * Try to keep it short, snappy and to the point.
#   * Write the description between the DESC delimiters below.
#   * Finally, don't worry about the indent, CocoaPods strips it!

  s.description      = <<-DESC
TODO: Add long description of the pod here.
                       DESC

  s.homepage         = 'https://github.com/novasamatech/hydra-math-swift'
  s.author           = { 'Ruslan Rezin' => 'ruslan@novawallet.io' }
  s.source           = { :git => 'https://github.com/svojsu/hydra-math-swift.git' }

  s.ios.deployment_target = '13.0'

  s.source_files = 'HydraMath/Classes/**/*', 'HydraMath/lib/include/**/*'
  s.public_header_files = 'HydraMath/lib/include/**/*.h'
  s.vendored_libraries = 'HydraMath/lib/libhydra_dx.a'
  
  # s.resource_bundles = {
  #   'HydraMath' => ['HydraMath/Assets/*.png']
  # }

  # s.public_header_files = 'Pod/Classes/**/*.h'
  # s.frameworks = 'UIKit', 'MapKit'
  # s.dependency 'AFNetworking', '~> 2.3'
end
