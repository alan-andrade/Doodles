class Level
  include Playable

  attr_reader :name

  def initialize config
    @name = config.fetch(:name)
    super
  end
end
