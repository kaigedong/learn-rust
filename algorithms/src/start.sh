for i in `/bin/ls ./`
do
    mv $i/lib.rs $i/mod.rs
    #echo $i
    #mv $i/src/* $i/ && rm $i/Cargo.toml && rm -r $i/src/
    #name=`echo $i|sed 's/-/_/g'`
    #mv $i $name
done