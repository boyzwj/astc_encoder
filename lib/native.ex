defmodule AstcEncoder.Native do
  use Rustler, otp_app: :astc_encoder, crate: :astcenc
  def add(_a, _b), do: :erlang.nif_error(:nif_not_load)

  def create(_data, _width, _height, _quality, _targetsize), do: :erlang.nif_error(:nif_not_load)
end
