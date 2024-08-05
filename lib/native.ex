defmodule AstcEncoder.Native do
  use Rustler, otp_app: :astc_encoder, crate: :astcenc

  def thumbnail(_data, _width, _height, _block_size, _speed) do
    :erlang.nif_error(:nif_not_load)
  end
end
