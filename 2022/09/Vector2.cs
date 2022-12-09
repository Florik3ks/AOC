public class Vector2{
    public int x, y;
    public Vector2(){
        this.x = 0;
        this.y = 0;
    }
    public Vector2(int x, int y){
        this.x = x;
        this.y = y;
    }
    // override object.Equals
    public override bool Equals(object obj)
    {
        //
        // See the full list of guidelines at
        //   http://go.microsoft.com/fwlink/?LinkID=85237
        // and also the guidance for operator== at
        //   http://go.microsoft.com/fwlink/?LinkId=85238
        //
        
        if (obj == null || GetType() != obj.GetType())
        {
            return false;
        }
        Vector2 o = (Vector2)obj;
        return o.x == x && o.y == y;
        return base.Equals (obj);
    }
    
    // override object.GetHashCode
    public override int GetHashCode()
    {
        // TODO: write your implementation of GetHashCode() here
        return base.GetHashCode();
    }
}