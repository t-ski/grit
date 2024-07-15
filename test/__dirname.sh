__DIRNAME=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)

dirname() {
    echo $__DIRNAME
}