import DateCell from '$lib/components/table/DateCell.svelte';
import PaginationControls from '$lib/components/table/PaginationControls.svelte';
import ParagraphCell from '$lib/components/table/ParagraphCell.svelte';
import TextCell from '$lib/components/table/TextCell.svelte';
import TitleCell from '$lib/components/table/TitleCell.svelte';
import {
	createPaginatedRowModel,
	createTableHook,
	rowPaginationFeature,
	tableFeatures
} from '@tanstack/svelte-table';

const _features = tableFeatures({
	rowPaginationFeature
});

/** @typedef {typeof _features} AppTableFeatures */

const cellComponents = {
	TitleCell,
	TextCell,
	ParagraphCell,
	DateCell
};

/** @typedef {typeof cellComponents} AppTableCellComponents */

const headerComponents =
	/** @type {Record<string, import('@tanstack/svelte-table').ComponentType<any>>} */ ({});

/** @typedef {typeof headerComponents} AppTableHeaderComponents */

/**
 * @template {import('@tanstack/svelte-table').RowData} T
 * @typedef {import('@tanstack/svelte-table').AppColumnHelper<AppTableFeatures, T, AppTableCellComponents, AppTableHeaderComponents>} AppTableColumnHelper
 */

const tableComponents = {
	PaginationControls
};

export const {
	createAppColumnHelper,
	createAppTable,
	useTableContext,
	useCellContext,
	useHeaderContext
} = createTableHook({
	_features,
	_rowModels: {
		paginatedRowModel: createPaginatedRowModel()
	},

	getRowId: (row) => row.id,

	cellComponents,
	headerComponents,
	tableComponents
});
