defmodule AstcEncoder.Util do
  @magic <<0x13, 0xAB, 0xA1, 0x5C>>
  def encode_dim(dim) do
    dim0 = Bitwise.band(dim, 0xFF)
    dim1 = Bitwise.band(Bitwise.bsr(dim, 8), 0xFF)
    dim2 = Bitwise.band(Bitwise.bsr(dim, 16), 0xFF)
    <<dim0, dim1, dim2>>
  end

  def pkg_data(data, block_size, w, h, c) do
    ew = encode_dim(w)
    eh = encode_dim(h)
    ec = encode_dim(c)
    [<<@magic::binary, block_size, block_size, 1, ew::binary, eh::binary, ec::binary>> | data]
  end
end
