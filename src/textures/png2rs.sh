#!/bin/sh

src_file="$1"
dest_file="${src_file%%.png}.rs"
tex_name="$(echo ${src_file%%.png} | tr '[:lower:]' '[:upper:]')_TEXTURE"
width=$(file $src_file | cut -d' ' -f5)
height=$(file $src_file | cut -d' ' -f7)
height=${height%%,} 
printf  "pub const ${tex_name}_WIDTH: usize = $width;\\npub const ${tex_name}_HEIGHT: usize = $height;\\npub const ${tex_name}: [[u8; 3]; $width * $height + 1] = [\\n" > $dest_file
2ff < "$src_file" \
	| hexdump -dv \
	| tail -n+2 \
	| head -n-1 \
	| awk '{printf "[%i,%i,%i],[%i,%i,%i],", \
	int($2/256), int($3/256), int($4/256), \
	int($6/256), int($7/256), int($8/256)}' \
	>> $dest_file
echo '];' >> $dest_file
