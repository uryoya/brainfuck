import scala.io.Source

object Main {
  val MEM_SIZE = 3000
  def main(args: Array[String]): Unit =  {
    if (0 >= args.length) {
      return
    } 
    val script = Source.fromFile(args(0)).toArray
    val memory = new Array[Int](MEM_SIZE)
    var pointer = 0;
    var idx = 0;
    while (idx < script.length) {
      script(idx) match {
        case '>' => pointer += 1
        case '<' => pointer -= 1
        case '+' => memory(pointer) += 1
        case '-' => memory(pointer) -= 1
        case '.' => print(memory(pointer).toChar)
        case ',' =>
        case '[' => if (memory(pointer) == 0) {
          var roop = 1
          while (roop > 0) {
            idx += 1
            script(idx) match {
              case ']' => roop -= 1
              case '[' => roop += 1
              case _ =>
            }
          }
        }
        case ']' => {
          var roop = 1
          while (roop > 0) {
            idx -= 1
            script(idx) match {
              case '[' => roop -= 1
              case ']' => roop += 1
              case _ =>
            }
          }
          idx -= 1
        }
        case _ =>
      }
      if (pointer < 0 || pointer > MEM_SIZE) {
        println("ぬるぽ")
        return
      }
      idx += 1
    }
    print('\n')
  }
}
