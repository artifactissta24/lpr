fn auto    (){
           [ 1 ;   Multiply                   :: VAL ] ;
}
trait TypeVal < VAL >{
const VAL : usize;
}
struct Five ;
struct Multiply < Five,Five>{
Five:VAL
}
impl            TypeVal< Five  > for Multiply < Five,Five>
where Five:TypeVal< VAL >
{
}
