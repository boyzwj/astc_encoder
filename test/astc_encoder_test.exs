defmodule AstcEncoderTest do
  use ExUnit.Case
  doctest AstcEncoder

  alias Vix.Vips.Image, as: VImage
  alias Vix.Vips.Operation, as: VOp

  @source_img "priv/test.png"

  test "greets the world" do
    {:ok, {img, _args}} = VOp.pngload(@source_img)
    {:ok, img} = VOp.resize(img, 0.5)
    assert AstcEncoder.hello() == :world
  end
end
