alias Vix.Vips
cli_path = :code.priv_dir(:astc_encoder) |> Path.join(["astcenc-avx2"])
System.cmd("chmod", ["+x", cli_path])
{:ok, origin_image} = File.read("priv/test.png")

t1 = fn ->
  Temp.track()
  temp_dir = Temp.mkdir!()
  temp_resize_path = Path.join([temp_dir, "test.png"])
  dst_name = "test.astc"
  {:ok, interpolate} = Vips.Interpolate.new("nearest")
  {:ok, {image, _flags}} = Vips.Operation.pngload_buffer(origin_image, disc: false, memory: true)
  {:ok,image}=Vips.Operation.resize(image, 0.5,
      kernel: :VIPS_KERNEL_NEAREST,
      interpolate: interpolate
    )
  Vips.Operation.pngsave(image, temp_resize_path, strip: true, compression: 6)
  temp_astc_path = Path.join([temp_dir, dst_name])
  System.cmd(cli_path, ["-cl", temp_resize_path, temp_astc_path, "8x8", "-medium", "-yflip"])
  {:ok, file_content} = File.read(temp_astc_path)
  <<_skip::binary-size(16), remaining_content::binary>> = file_content
  remaining_content
end

t2 = fn ->
  AstcEncoder.Native.thumbnail(origin_image, 512, 512, 8, 3)
end

Benchee.run(
  %{
    "cli" => t1,
    "nif" => t2
  },
  time: 10,
  memory_time: 5,
  warmup: 2,
  # formatters: [Benchee.Formatters.Console],
  parallel: 4
)
