use enumx::export::*;
use enumx::predefined::*;
use cex::*;

use crate::{
    InterpResult,
    PathOptsWidgets,
    TkBBoxTrait,
    TkInstance,
    TkOption,
    TkXView,
    TkXViewIndex,
    TkYView,
    TkYViewIndex,
    Widget,
    opt::{
        OptPair,
        TkListboxItemOpt,
    },
    range::{
        TkDefaultStart,
        TkDefaultEnd,
        TkRange,
    },
};

use std::{
    ops::{
        RangeFrom,
        RangeInclusive,
        RangeToInclusive,
    },
};

use tcl::{
    Obj,
    error::{
        DeError,
        InterpError,
        NotList,
    },
    from_obj,
};

use tuplex::*;

#[derive( Copy, Clone )]
pub struct TkListbox<Inst:TkInstance>( pub(crate) Widget<Inst> );

pub enum Index {
    Number( clib::Tcl_Size ),
    Active,
    Anchor,
    End,
    At( clib::Tcl_Size ),
}

impl From<clib::Tcl_Size> for Index {
    fn from( number: clib::Tcl_Size ) -> Self { Index::Number( number )}
}

impl TkDefaultStart for Index {
    fn default_start() -> Self { Index::Number(0) }
}

impl TkDefaultEnd for Index {
    fn default_end() -> Self { Index::End }
}

impl From<RangeFrom<clib::Tcl_Size>> for TkRange<Index> { // a..
    fn from( r: RangeFrom<clib::Tcl_Size> ) -> Self {
        TkRange {
            start : Index::Number( r.start ),
            end   : Index::default_end()
        }
    }
}

impl From<RangeInclusive<clib::Tcl_Size>> for TkRange<Index> { // a..=b
    fn from( r: RangeInclusive<clib::Tcl_Size> ) -> Self {
        TkRange {
            start : Index::Number( *r.start() ),
            end   : Index::Number( *r.end() )
        }
    }
}

impl From<RangeToInclusive<clib::Tcl_Size>> for TkRange<Index> { // ..=b
    fn from( r: RangeToInclusive<clib::Tcl_Size> ) -> Self {
        TkRange {
            start : Index::default_start(),
            end   : Index::Number( r.end ),
        }
    }
}

impl From<Index> for Obj {
    fn from( index: Index ) -> Obj {
        use Index::*;
        match index {
            Number( n ) => n                  .into(),
            Active      => "active"           .into(),
            Anchor      => "anchor"           .into(),
            End         => "end"              .into(),
            At( n )     => format!( "@{}", n ).into(),
        }
    }
}

impl<Inst:TkInstance> TkListbox<Inst> {
    pub fn active( &self, index: impl Into<Index> ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "active", index.into() ))
    }

    #[cex]
    pub fn curselection( &self ) -> Result!( Vec<clib::Tcl_Size> throws DeError, InterpError ) {
        let obj = self.0.tk().eval(( self.0.path, "curselection" ))?;
        ret!( from_obj::<Vec<clib::Tcl_Size>>( obj ));
    }

    /// Deletes one element of the listbox.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tcl::*;
    /// use tk::*;
    /// use tk::cmd::*;
    ///
    /// fn concat( v: Vec<Obj> ) -> String {
    ///     v.iter().map( |obj| obj.get_string() ).collect::<String>()
    /// }
    ///
    /// fn main() -> TkResult<()> {
    ///     let tk = make_tk!()?;
    ///     let root = tk.root();
    ///     tk.set( "items", "a b c d e f g h i" );
    ///     let lbox = root.add_listbox( "lbox" -listvariable("items") )?;
    ///
    ///     lbox.delete(1)?;
    ///     assert_eq!( concat( lbox.get_range(..)? ), "acdefghi" );
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn delete( &self, index: impl Into<Index> ) -> InterpResult<()> {
        self.tk().run(( self.path, "delete", index.into() ))
    }

    /// Deletes one or more elements of the listbox.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tcl::*;
    /// use tk::*;
    /// use tk::cmd::*;
    ///
    /// fn concat( v: Vec<Obj> ) -> String {
    ///     v.iter().map( |obj| obj.get_string() ).collect::<String>()
    /// }
    ///
    /// fn main() -> TkResult<()> {
    ///     let tk = make_tk!()?;
    ///     let root = tk.root();
    ///     tk.set( "items", "a b c d e f g h i" );
    ///     let lbox = root.add_listbox( "lbox" -listvariable("items") )?;
    ///
    ///     lbox.delete_range(1..=1)?;
    ///     assert_eq!( concat( lbox.get_range(..)? ), "acdefghi" );
    ///
    ///     lbox.delete_range(6..)?;
    ///     assert_eq!( concat( lbox.get_range(..)? ), "acdefg" );
    ///
    ///     lbox.delete_range(..=1)?;
    ///     assert_eq!( concat( lbox.get_range(..)? ), "defg" );
    ///
    ///     lbox.delete_range(1..=2)?;
    ///     assert_eq!( concat( lbox.get_range(..)? ), "dg" );
    ///
    ///     lbox.delete_range(..)?;
    ///     assert_eq!( concat( lbox.get_range(..)? ), "" );
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn delete_range( &self, range: impl Into<TkRange<Index>> ) -> InterpResult<()> {
        let range = range.into();
        self.tk().run(( self.path, "delete", range.start, range.end ))
    }

    /// Returns the contents of the listbox element indicated by the index, or an empty
    /// string if first refers to a non-existent element.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tcl::*;
    /// use tk::*;
    /// use tk::cmd::*;
    ///
    /// fn main() -> TkResult<()> {
    ///     let tk = make_tk!()?;
    ///     let root = tk.root();
    ///
    ///     tk.set( "items", "alpha beta gamma" );
    ///     let lbox = root.add_listbox( "lbox" -listvariable("items") -height(5) )?.pack(())?;
    ///     let expected = vec![ "alpha".to_owned(), "beta".to_owned(), "gamma".to_owned() ];
    ///
    ///     assert_eq!( lbox.get(0)?.get_string(), expected[0] );
    ///     assert_eq!( lbox.get(1)?.get_string(), expected[1] );
    ///     assert_eq!( lbox.get(2)?.get_string(), expected[2] );
    ///     assert_eq!( lbox.get(3)?.get_string(), "" );
    ///
    ///     assert_eq!( lbox.get( TkListboxIndex::End )?.get_string(), expected[2] );
    ///
    ///     Ok(())
    /// }
    /// ```
    #[cex]
    pub fn get( &self, index: impl Into<Index> ) -> Result!( Obj throws InterpError, NotList ) {
        Ok( self.0.tk().eval(( self.0.path, "get", index.into() ))? )
    }

    ///  Returns a list whose elements are all of the listbox elements in the range.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tcl::*;
    /// use tk::*;
    /// use tk::cmd::*;
    ///
    /// fn main() -> TkResult<()> {
    ///     let tk = make_tk!()?;
    ///     let root = tk.root();
    ///
    ///     tk.set( "items", "alpha beta gamma" );
    ///     let lbox = root.add_listbox( "lbox" -listvariable("items") -height(5) )?.pack(())?;
    ///     let expected = vec![ "alpha".to_owned(), "beta".to_owned(), "gamma".to_owned() ];
    ///
    ///     assert_eq!( &lbox.get_range( ..  )?.iter().map( |obj| obj.get_string() ).collect::<Vec<_>>(), &expected );
    ///     assert_eq!( &lbox.get_range(0..  )?.iter().map( |obj| obj.get_string() ).collect::<Vec<_>>(), &expected );
    ///     assert_eq!( &lbox.get_range( ..=2)?.iter().map( |obj| obj.get_string() ).collect::<Vec<_>>(), &expected );
    ///     assert_eq!( &lbox.get_range(0..=2)?.iter().map( |obj| obj.get_string() ).collect::<Vec<_>>(), &expected );
    ///
    ///     assert_eq!( &lbox.get_range(1..=2)?.iter().map( |obj| obj.get_string() ).collect::<Vec<_>>(), &expected[1..=2] );
    ///
    ///     Ok(())
    /// }
    /// ```
    #[cex]
    pub fn get_range( &self, range: impl Into<TkRange<Index>> ) -> Result!( Vec<Obj> throws InterpError, NotList ) {
        let range = range.into();
        let obj = self.0.tk().eval(( self.0.path, "get", range.start, range.end ))?;
        Ok( obj.get_elements()?.collect::<Vec<_>>() )
    }

    pub fn index( &self, index: impl Into<Index> ) -> InterpResult<clib::Tcl_Size> {
        let result = self.0.tk().eval(( self.0.path, "index", index.into() ))?;
        self.0.tk().tclsize( result )
    }

    pub fn insert( &self, index: impl Into<Index>, elements: impl IntoIterator<Item=Obj> ) -> InterpResult<()> {
        let mut command = IntoVec::<Obj>::into_vec(( self.0.path, "insert", index.into() ));
        command.extend( elements );
        self.0.tk().run( command )
    }

    pub fn insert_end( &self, elements: impl IntoIterator<Item=Obj> ) -> InterpResult<()> {
        let mut command = IntoVec::<Obj>::into_vec(( self.0.path, "insert", "end" ));
        command.extend( elements );
        self.0.tk().run( command )
    }

    pub fn itemcget<Opt,Val>( &self, index: impl Into<Index>, _name_fn: fn(Val)->Opt ) -> InterpResult<Obj>
        where Opt : TkOption
                  + Into<TkListboxItemOpt>
            , Val : Into<Obj>
    {
        self.0.tk().eval(( self.0.path, "itemcget", index.into(), <Opt as TkOption>::NAME ))
    }

    pub fn itemconfigure<Opts>( &self, index: impl Into<Index>, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
        where Opts: IntoHomoTuple<TkListboxItemOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 3 );
        command.push( self.path.into() );
        command.push( "itemconfigure".into() );
        command.push( Obj::from( index.into() ));
        crate::cmd::append_opts( &mut command, opts.into().opts );
        self.tk().run( command )
    }

    pub fn nearest( &self, y: clib::Tcl_Size ) -> InterpResult<clib::Tcl_Size> {
        let obj = self.0.tk().eval(( self.0.path, "nearest", y ))?;
        self.0.tk().tclsize( obj )
    }

    pub fn scan_mark( &self, x: clib::Tcl_Size, y: clib::Tcl_Size ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "scan", "mark", x, y ))
    }

    pub fn scan_dragto( &self, x: clib::Tcl_Size, y: clib::Tcl_Size ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "scan", "dragto", x, y ))
    }

    pub fn see( &self, index: impl Into<Index> ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "see", index.into() ))
    }

    pub fn selection_anchor( &self, index: impl Into<Index> ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "selection", "anchor", index.into() ))
    }

    pub fn selection_clear( &self, index: impl Into<Index> ) -> InterpResult<()> {
        self.tk().run(( self.path, "selection", "clear", index.into() ))
    }

    pub fn selection_clear_range( &self, range: impl Into<TkRange<Index>> ) -> InterpResult<()> {
        let range = range.into();
        self.tk().run(( self.path, "selection", "clear", range.start, range.end ))
    }

    pub fn selection_includes( &self, index: impl Into<Index> ) -> InterpResult<bool> {
        let obj = self.0.tk().eval(( self.0.path, "selection", "includes", index.into() ))?;
        self.0.tk().boolean( obj )
    }

    pub fn selection_set( &self, index: impl Into<Index> ) -> InterpResult<()> {
        self.tk().run(( self.path, "selection", "set", index.into() ))
    }

    pub fn selection_set_range( &self, range: impl Into<TkRange<Index>> ) -> InterpResult<()> {
        let range = range.into();
        self.tk().run(( self.path, "selection", "set", range.start, range.end ))
    }

    pub fn size( &self ) -> InterpResult<clib::Tcl_Size> {
        let obj = self.0.tk().eval(( self.0.path, "size" ))?;
        self.0.tk().tclsize( obj )
    }
}

impl<Inst:TkInstance> TkBBoxTrait<Inst> for TkListbox<Inst> {
    type Index = Index;
}

impl<TK:TkInstance> TkXView<TK> for TkListbox<TK> {}

impl<TK:TkInstance> TkXViewIndex<TK> for TkListbox<TK> {
    type Index = Index;
}

impl<TK:TkInstance> TkYView<TK> for TkListbox<TK> {}

impl<TK:TkInstance> TkYViewIndex<TK> for TkListbox<TK> {
    type Index = Index;
}
