require 'spec_helper'
require 'playable'

describe Playable do
  describe '#play' do
    class TestTrack
      include Playable
    end

    it 'evaluates the challenge proc and returns boolean' do
      track = TestTrack.new(challenge: ->(params) do
        params.fetch(:has_full_access)
      end)

      params = Playable::Params.new(has_full_access: true)
      expect(track.play(params)).to be_true

      params = Playable::Params.new(has_full_access: false)
      expect(track.play(params)).to be_false
    end
  end

  describe Playable::PSet do
    class TestLevel
      include Playable
    end

    describe '#play' do
      it 'evaluates the challenge proc for each member, returns the first match' do
        foo_level = TestLevel.new challenge: ->(params) { params[:foo] }
        bar_level = TestLevel.new challenge: ->(params) { params[:bar] }
        levels = Playable::PSet.new([foo_level, bar_level])

        params = Playable::Params.new(foo: true, bar: false)
        expect(levels.play(params)).to eq foo_level

        params = Playable::Params.new(foo: false, bar: true)
        expect(levels.play(params)).to eq bar_level
      end

      it 'raise exception when no level found' do
        levels = Playable::PSet.new([])
        expect {
          levels.play(Playable::Params.new)
        }.to raise_error(Playable::NoMatchException)
      end
    end
  end
end

