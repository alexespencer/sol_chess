if [ ! -d games ]; then
    mkdir games
fi

for i in {1..10}; do
    echo "*********** Game $i ************" >> games/$i.txt
    echo "" >> games/$i.txt
    sol_cli -g -n 7 --print >> games/$i.txt
done
