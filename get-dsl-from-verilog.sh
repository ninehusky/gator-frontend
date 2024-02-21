# Check the number of arguments
if [ "$#" -ne 2 ]; then
    echo "Illegal number of parameters"
    echo "Usage: ./get-dsl-from-sv.sh <verilog_file> <output_file>"
    echo "Example: ./get-dsl-from-sv.sh test.sv test.dsl"
    exit 1
fi

# Invoke yosys
yosys -q -p "plugin -i churchroad; read_verilog -sv $1; write_lakeroad" > $2
