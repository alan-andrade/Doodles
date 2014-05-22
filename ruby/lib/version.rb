class Version
  include Playable

  attr_reader :tracks

  def initialize config
    @tracks = Playable::PSet.new(config.fetch(:tracks, []))
    super
  end
end
