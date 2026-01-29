/**
 * MetadataTooltip - Displays block metadata on hover.
 *
 * Shows ID, type, role, tags, timestamps, and custom metadata.
 */
import React from 'react';
import type { Block } from 'ucp-content';
export interface MetadataTooltipProps {
    block: Block;
    onClose: () => void;
}
/**
 * Displays detailed metadata for a block.
 */
export declare function MetadataTooltip({ block, onClose }: MetadataTooltipProps): React.ReactElement;
export default MetadataTooltip;
