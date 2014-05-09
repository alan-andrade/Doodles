module Playable
  def initialize config
    # challenge is a block that will be evaluated passing the "params".
    #
    # You put logic here that if returns a truthy result, it will "match" and
    # pick that entity (level/track)
    @challenge = config.fetch(:challenge)
  end

  def play(params)
    params.check!
    @challenge.call(params)
  end

  class NoMatchException < Exception
    def message
      "No match for any challenge was found"
    end
  end

  class PSet < Set
    def play(params)
      find {|t| t.play(params) } || raise(NoMatchException)
    end
  end

  class Params < Hash
    def initialize(*args, &block)
      passed_hash = args.find {|a| a.is_a? Hash }
      if passed_hash
        args.delete(passed_hash)
        super(*args, &block)
        merge! passed_hash
      else
        super(*args, &block)
      end
    end

    def check!
      # probably check that user exist or any other key-value exists.
    end
  end
end
