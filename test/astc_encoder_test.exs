defmodule AstcEncoderTest do
  use ExUnit.Case
  doctest AstcEncoder

  @source_img "priv/test.png"

  test "greets the world" do
    {:ok, data} = File.read(@source_img)
    output = AstcEncoder.Native.thumbnail(data, 512, 512, 6, 3)
    file_data = AstcEncoder.Util.pkg_data(output, 6, 512, 512, 1)
    :ok = File.write("priv/test.astc", file_data)
    assert AstcEncoder.hello() == :world
  end
end
