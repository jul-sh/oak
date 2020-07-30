Require Import OakIFC.Lattice.
Require Import OakIFC.Parameters.
Require Import OakIFC.GenericMap.
Require Import List.
Import ListNotations.
Require Import Coq.Sets.Ensembles.

(* Ensembles don't have implicit type params and these lines fix that *)
Arguments Ensembles.In {U}.
Arguments Ensembles.Add {U}.
Arguments Ensembles.Subtract {U}.
Arguments Ensembles.Singleton {U}.

(* TODO:
    - distinguish between read/write handles
    - use option types for channel_state node_state range
    - look into libraries for record types
*)
(*============================================================================
 Commands, State, Etc.
============================================================================*)
Record channel := Chan {
    clbl: level;
    ms: list message
}.

(* ABI Calls *)
Inductive call: Type :=
    | WriteChannel (h: handle) (m: message): call
    | ReadChannel (h: handle): call
    | CreateChannel (lbl: level)(wid: node_id)(rid: node_id): call
    | CreateNode (lbl: level)(h: handle): call
    | Internal: call.
(* TODO wait_on_channels, channel_close *)

Record node := Node {
    nlbl: level;
    hans: Ensemble handle;
    ncall: call
}.

Instance Knid: KeyT := {
    t := node_id; 
    eq_dec := dec_eq_nid;
}.
Instance Khandle: KeyT := {
    t := handle;
    eq_dec := dec_eq_h;
}.
Definition node_state := tg_map Knid node.
Definition chan_state := tg_map Khandle channel.
Record state := State {
    nodes: node_state;
    chans: chan_state
}.

(*============================================================================
* Empty
============================================================================*)
Definition empty_chan := {| clbl := top; ms := []; |}.
Definition empty_node := {|
        nlbl := top;
        hans := Empty_set handle;
        ncall := Internal;
    |}.
Definition empty_state := {| 
        nodes := ( _ !-> empty_node);
        chans := ( _ !-> empty_chan);
    |}.

(*============================================================================
* Utils
============================================================================*)
Definition chan_append (c: channel)(m: message): channel :=
    {| clbl := c.(clbl); ms := (m :: c.(ms)) |}.

(* this is used in channel read where there is a premise
* that checks that the channel is not empty *)
Definition chan_pop (c: channel): channel :=
    {| 
        clbl := c.(clbl); 
        ms := match c.(ms) with
            | nil => nil
            | m :: ms' => ms'
        end;
    |}.

Definition state_upd_node (nid: node_id)(n: node)(s: state): state :=
    {| 
        nodes := tg_update s.(nodes) nid n; 
        chans := s.(chans)
    |}.


Definition state_upd_chan (h: handle)(ch: channel)(s: state): state :=
    {|
        nodes := s.(nodes);
        chans := tg_update s.(chans) h ch;
    |}.

Definition state_upd_call (nid: node_id)(c: call)(s: state): state :=
    let old_n := (s.(nodes) nid) in
    state_upd_node nid ({|
            nlbl := old_n.(nlbl);
            hans := old_n.(hans);
            ncall := c;
        |}) s.

Definition state_append_msg (h: handle)(m: message)(s: state): state :=
    state_upd_chan h (chan_append (s.(chans) h) m) s.

Definition state_chan_pop (h: handle)(s: state): state :=
    state_upd_chan h (chan_pop (s.(chans) h)) s.

Definition state_node_add_han (h: handle)(nid: node_id)(s: state): state :=
    let old_n := (s.(nodes) nid) in
    state_upd_node nid {|
            nlbl  := old_n.(nlbl);
            hans  := Ensembles.Add old_n.(hans) h;
            ncall := old_n.(ncall);
        |} s.

(* 
TODO it would actually be better to use option types
for the range of both the node state and channel states 
*)
(* There may be potential problems with these definitions *)
Definition handle_fresh (s: state)(h: handle): Prop :=
    (s.(chans) h) = empty_chan.

Definition nid_fresh (s: state)(nid: node_id): Prop :=
    (s.(nodes) nid) = empty_node.

(*============================================================================
* Single Call Semantics
============================================================================*)

(* step for a single node (which can be thought of as a thread) *)
Inductive step_node: node_id -> call -> state -> state -> Prop :=
    | SWriteChan s id n han msg
            (* it might be awkward looking that there is no premise checking
            that the call is really the call of this particular node. 
            The global transition relation checks this, though. It could
            be added redundantly here ? Another option could be to make this
            a node -> state -> state -> Prop relation, that checks this
            redundantly. Then have a separate relation with the -> call -> bit
            to make proofs easier, then prove they are equivalent *)
        (H0: (s.(nodes) id) = n)
        (H1: In n.(hans) han)
        (H2: n.(nlbl) << (s.(chans) han).(clbl)):
        step_node id (WriteChannel han msg) s (state_append_msg han msg s)
    | SReadChan s id n han chan
            (* note that, we already expect that this call 
            * will have leaks and we cannot prove NI with it *)
        (H0: (s.(nodes) id) = n)
        (H1: In n.(hans) han)
        (H2: (s.(chans) han) = chan)
        (H3: (length chan.(ms)) > 0)
        (H4: chan.(clbl) << n.(nlbl)):
        step_node id (ReadChannel han) s (state_chan_pop han s)
    | SCreateChan s cid rid wid h lbl
        (H1: (s.(nodes) cid).(nlbl) << lbl)
            (* in an alternative design, there could be checks here comparing 
            the labels of the reader/writer and the label of the channel 
            here *instead* of in the read/write calls *)
        (H2: handle_fresh s h):
            let s0 := (state_upd_chan h {| ms := []; clbl := lbl; |} s) in
            let s1 := (state_node_add_han h rid s0) in
            let s' := (state_node_add_han h wid s1) in
            step_node cid (CreateChannel lbl rid wid) s s'
    | SCreateNode s cid nid lbl h
        (H0: (s.(nodes) cid).(nlbl) << lbl)
        (* consider the following also : *)
        (* (H1: lbl << (s.(chans). h).(clbl) ) *)
        (H1: (nid_fresh s nid)):
        step_node cid (CreateNode lbl h) s 
            (state_upd_node nid {| 
                nlbl := lbl;
                hans := (Singleton h);
                ncall := Internal;
            |} s)
    | SInternal s id: step_node id Internal s s.

(* step for the full system (which picks a thread to execute and is
non-deterministic). This is needed in addition to step_node, because
we should show that regardless of the thread scheduling, there are no information
leaks. *)
(* To be general and language agnostic, computation of code within nodes other
than the ABI calls is modeled as simply returning an arbitrary continuation 
(c') of the node's choosing (for any call). *)
(* Errors might later be modeled by choosing a different continuation based
on whether or not a call was successful, in this case, the resulting
continuation likely needs to be moved into the local transition relation *)
Inductive step_system: state -> state -> Prop :=
    (* possibly also a termination case *)
    | ValidStep id n c c' s s'
        (H0: (s.(nodes) id) = n)
        (H1: n.(ncall) = c)
        (H2: step_node id c s s'):
        step_system s (state_upd_call id c' s').

(* Traces are sequences of states modeling entire executions *)
(* Noninterference definitions are defined by comparing "any two" executions *)
Definition trace := list state.

Definition last_state (t: trace)(s: state): Prop :=
    match t with
        | [] => False
        | x :: t' => s = x
    end.

Inductive valid_trace (init: state): trace -> Prop :=
    | VTOne: (valid_trace init [init] )
    | VTAdd (s s': state)(t: trace)
            (H0: valid_trace init t )
            (H1: last_state t s')
            (H2: step_system s s):
            valid_trace init (s' :: t).
