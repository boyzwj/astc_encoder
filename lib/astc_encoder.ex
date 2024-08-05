defmodule AstcEncoder do
  alias AstcEncoder.Native
  alias AstcEncoder.Util

  @moduledoc """
  Documentation for `AstcEncoder`.
  """

  @doc """
  Hello world.

  ## Examples

      iex> AstcEncoder.hello()
      :world

  """
  def hello do
    :world
  end

  @doc """
  make thumbnail astc raw data

  thumbnail(data, width, height, block_size, speed)

  block size must in [4,5,6,8]
  speed :
      1  fastest
      2 fast
      3 medium
      4 thorough
      5 very thorough
      6 exhaustive
  """
  def thumbnail(data, width, height, block_size, speed) do
    if block_size not in [4, 5, 6, 8] do
      raise "block size must be in [4,5,6,8]"
    end

    if speed not in [1, 2, 3, 4, 5, 6] do
      raise "spedd must be in [1,2,3,4,5,6"
    end

    Native.thumbnail(data, width, height, block_size, speed)
  end

  def thumbnail_file(data, width, height, block_size, speed) do
    if block_size not in [4, 5, 6, 8] do
      raise "block size must be in [4,5,6,8]"
    end

    if speed not in [1, 2, 3, 4, 5, 6] do
      raise "spedd must be in [1,2,3,4,5,6"
    end

    case Native.thumbnail(data, width, height, block_size, speed) do
      {:error, reason} ->
        {:error, reason}

      data ->
        Util.pkg_data(data, block_size, width, height, 1)
    end
  end
end
