/// Different access permissions, such as `pub` and `priv`.
#[derive(Clone, Debug, PartialEq)]
pub enum Access {

    /// A public access item, which may be accessed anywhere inside or outside of the module.
    Pub,

    /// `priv` allows access only to the current scope, for example:
    /// 
    /// ```flycatcher
    /// // module1.flyc
    /// priv @func item(): string {
    ///     "Hello, world!"
    /// }
    /// 
    /// // module2.flyc
    /// #import "module1.flyc"
    /// 
    /// @func main() {
    ///     my_var = item() // ERROR!
    /// }
    /// ```
    Priv

}