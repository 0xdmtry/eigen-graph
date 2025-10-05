"use client";
import React, {useEffect, useMemo, useRef, useState} from "react";
import {DndContext, PointerSensor, KeyboardSensor, useSensor, useSensors, closestCenter} from "@dnd-kit/core";
import {
    SortableContext,
    useSortable,
    arrayMove,
    sortableKeyboardCoordinates,
    rectSwappingStrategy,
} from "@dnd-kit/sortable";
import type {TableItem} from "@/types/operators";
import {baseTokenCards} from "@/data/tokens";
import {useToken} from "@/context/TokenContext";
import TokenAutocomplete from "@/components/tokens/TokenAutocomplete";
import Badge from "@/components/ui/badge/Badge";
import TokenCardDnD from "./TokenCardDnD";
import {TokenCardDnDType} from "@/types/tokens";


const STORAGE_KEY = "tokenPanelOrder:v1";
const CARDS_SHOWN_COLLAPSED = 6;

const sortByOpsThenSymbol = (cards: TokenCardDnDType[]) =>
    [...cards].sort((a, b) => (b.operators - a.operators) || a.symbol.localeCompare(b.symbol)).map(c => c.symbol);

function buildCards(tokens: Record<string, TableItem[]>): TokenCardDnDType[] {
    return baseTokenCards.map(({symbol, name, icon}) => {
        const rows = tokens[symbol] || [];
        const operators = rows.length;
        const tvl = rows.reduce((s, r) => {
            try {
                return s + BigInt(r.tvlTotalAtomic);
            } catch {
                return s;
            }
        }, BigInt(0)).toString();
        return {id: symbol, symbol, name, icon, tvl, operators};
    });
}

export default function TokenPanelDnDStable({tokens}: { tokens: Record<string, TableItem[]> }) {
    const {selectedTokenSymbol, setSelectedTokenSymbol} = useToken();
    const [isExpanded, setIsExpanded] = useState(false);
    const [order, setOrder] = useState<string[]>([]);
    const initializedRef = useRef(false);
    const cards = useMemo(() => buildCards(tokens), [tokens]);

    useEffect(() => {
        if (initializedRef.current) return;
        initializedRef.current = true;
        try {
            const saved = JSON.parse(localStorage.getItem(STORAGE_KEY) || "[]") as string[];
            const current = new Set(cards.map(c => c.symbol));
            if (Array.isArray(saved) && saved.length && saved.every(s => current.has(s))) {
                setOrder(saved);
            } else {
                setOrder(sortByOpsThenSymbol(cards));
            }
        } catch {
            setOrder(sortByOpsThenSymbol(cards));
        }
    }, [cards]);

    const orderedCards = useMemo(
        () => order.map(s => cards.find(c => c.symbol === s)).filter(Boolean) as TokenCardDnDType[],
        [order, cards]
    );

    const sensors = useSensors(
        useSensor(PointerSensor, {activationConstraint: {distance: 6}}),
        useSensor(KeyboardSensor, {coordinateGetter: sortableKeyboardCoordinates})
    );

    function SortableItem({card}: { card: TokenCardDnDType }) {
        const {
            setNodeRef,
            attributes,
            listeners,
            transform,
            transition,
            setActivatorNodeRef,
            isDragging
        } = useSortable({id: card.id});
        const style: React.CSSProperties = {
            transform: transform ? `translate3d(${transform.x}px, ${transform.y}px, 0)` : undefined,
            transition,
            opacity: isDragging ? 0.9 : 1
        };
        return (
            <div ref={setNodeRef} style={style}>
                <TokenCardDnD
                    id={card.id}
                    symbol={card.symbol}
                    name={card.name}
                    icon={card.icon}
                    tvl={card.tvl}
                    operators={card.operators}
                    isActive={card.symbol.toUpperCase() === (selectedTokenSymbol || "").toUpperCase()}
                    onSelect={() => setSelectedTokenSymbol(card.symbol)}
                    dragHandleRef={setActivatorNodeRef as React.Ref<HTMLButtonElement>}
                    dragHandleAttributes={attributes}
                    dragHandleListeners={listeners}
                />
            </div>
        );
    }

    const canExpand = orderedCards.length > CARDS_SHOWN_COLLAPSED;

    if (!order.length) return null;

    return (
        <div className="rounded-2xl dark:border-gray-800 dark:bg-white/[0.03]">
            <div className="px-4 py-4 sm:pl-6 sm:pr-4">
                <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
                    <h3 className="text-lg font-semibold text-gray-800 dark:text-white/90">
                        All Tokens <Badge variant="solid" color="dark">{baseTokenCards.length}</Badge>
                    </h3>
                    <div className="flex flex-col gap-3 sm:flex-row sm:items-center">
                        <TokenAutocomplete/>
                    </div>
                </div>
            </div>
            <div className="dark:border-gray-800">
                <DndContext sensors={sensors} collisionDetection={closestCenter} onDragEnd={({active, over}) => {
                    if (!over || active.id === over.id) return;
                    const oldIndex = order.indexOf(String(active.id));
                    const newIndex = order.indexOf(String(over.id));
                    if (oldIndex === -1 || newIndex === -1) return;
                    const next = arrayMove(order, oldIndex, newIndex);
                    setOrder(next);
                    localStorage.setItem(STORAGE_KEY, JSON.stringify(next));
                }}>
                    <SortableContext items={order} strategy={rectSwappingStrategy}>
                        <div
                            className={`overflow-hidden transition-[max-height] duration-700 ease-in-out ${
                                isExpanded || !canExpand ? "max-h-[4000px]" : "max-h-[220px]"
                            }`}
                        >
                            <div className="p-4 sm:p-6 grid grid-cols-1 gap-4 sm:grid-cols-2 xl:grid-cols-3">
                                {orderedCards.map(card => <SortableItem key={card.id} card={card}/>)}
                            </div>
                        </div>
                    </SortableContext>
                </DndContext>
                {canExpand && (
                    <div className="p-2 dark:border-gray-800">
                        <button
                            onClick={() => setIsExpanded(v => !v)}
                            className="flex w-full items-center justify-center gap-2 rounded-lg py-2 text-sm font-medium text-gray-600 transition-colors hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-white/5"
                        >
                            <span>{isExpanded ? "Show Less" : "Show More"}</span>
                            <svg
                                className={`transform transition-transform duration-300 ${isExpanded ? "rotate-180" : ""}`}
                                width="16" height="16" viewBox="0 0 16 16" fill="none"
                            >
                                <path d="M4 6L8 10L12 6" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round"
                                      strokeLinejoin="round"/>
                            </svg>
                        </button>
                    </div>
                )}
            </div>
        </div>
    );
}
