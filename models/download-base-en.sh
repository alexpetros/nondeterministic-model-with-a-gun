#!/bin/bash

# Downloads the base-en Whisper model in ggml format.
# Adapted from: https://github.com/ggerganov/whisper.cpp/blob/09e90680072d8ecdf02eaf21c393218385d2c616/models/download-ggml-model.sh

src="https://huggingface.co/datasets/ggerganov/whisper.cpp"
pfx="resolve/main/ggml"

# get the path of this script
function get_script_path() {
  if [ -x "$(command -v realpath)" ]; then
    echo "$(dirname $(realpath $0))"
  else
    local ret="$(cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P)"
    echo "$ret"
  fi
}

models_path="$(get_script_path)"

# Whisper models
model="base.en"

# download ggml model
printf "Downloading ggml model $model from '$src' ...\n"

cd $models_path

if [ -f "ggml-$model.bin" ]; then
  printf "Model $model already exists. Skipping download.\n"
  exit 0
fi

if [ -x "$(command -v wget)" ]; then
  wget --quiet --show-progress -O ggml-$model.bin $src/$pfx-$model.bin
elif [ -x "$(command -v curl)" ]; then
  curl -L --output ggml-$model.bin $src/$pfx-$model.bin
else
  printf "Either wget or curl is required to download models.\n"
  exit 1
fi


if [ $? -ne 0 ]; then
  printf "Failed to download ggml model $model \n"
  printf "Please try again later or download the original Whisper model files and convert them yourself.\n"
  exit 1
fi

printf "Done! Model '$model' saved in 'models/ggml-$model.bin'\n"
