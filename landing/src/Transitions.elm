module Transitions exposing (transitions)

import Spa.Transition as Transition
import Utils.Spa as Spa


transitions : Spa.Transitions msg
transitions =
    { layout = Transition.fadeHtml 0
    , page = Transition.fadeHtml 0
    , pages = []
    }
