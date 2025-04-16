WAKEWORD="wakeword" # [kobe], bathroom, closet, kitchen, laundry, livingroom, outside, room, sink, workspace
WAKEWORD_FILENAME="${WAKEWORD// /_}"
# loop recordings, waiting one second after each.
for i in {0..1}; do
    (rustpotter-cli record --device-index 1 train/\[$WAKEWORD_FILENAME\]$i.wav && sleep 1) # [kobe]0.wav or bathroom0.wav or outside0.wav, etc.
    #ffmpeg -i train/\[$WAKEWORD_FILENAME\]$i.wav -ss 0 -t 1 \[$WAKEWORD_FILENAME\]$i.wav
done
# rustpotter-cli train -t small --train-dir train --test-dir test --test-epochs 10 --epochs 2500 -l 0.017 model.rpw
# rustpotter-cli spot -g --gain-ref 0.004 model.rpw
